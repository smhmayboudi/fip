use crate::{
    at::{proto::client::AtSaveReq, service::Service as AtService},
    auth::{
        auth_claims::AuthClaims,
        config::Config,
        proto::server::{
            AuthLoginReq, AuthLoginRes, AuthLogoutReq, AuthLogoutRes, AuthTokenReq, AuthTokenRes,
        },
    },
    jwks::{proto::client::JwksFindOneRandomReq, service::Service as JwksService},
    rt::{
        proto::client::{RtSaveReq, RtValidateReq},
        service::Service as RtService,
    },
    user::{proto::client::UserFindOneByUsernameReq, service::Service as UserService},
};
use chrono::Utc;
use fip_common::error::Error;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use tonic::Status;
use uuid::Uuid;

/// TODO: documentation
#[derive(Debug)]
pub struct Service {
    config: Config,
    access_token_service: AtService,
    jwks_service: JwksService,
    reference_token_service: RtService,
    user_service: UserService,
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[must_use]
    pub const fn new(
        config: Config,
        access_token_service: AtService,
        jwks_service: JwksService,
        reference_token_service: RtService,
        user_service: UserService,
    ) -> Self {
        Self {
            config,
            access_token_service,
            jwks_service,
            reference_token_service,
            user_service,
        }
    }
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn login(&self, req: &AuthLoginReq) -> Result<AuthLoginRes, Status> {
        let user = self
            .user_service
            .find_one_by_username(
                &UserFindOneByUsernameReq {
                    username: req.username.clone(),
                },
                "00000000-0000-0000-0000-000000000000",
            )
            .await?;
        if self.hash(req.password.clone()) != user.password {
            panic!("password incorrect.");
        }
        let at = self.access_token(&user.id).await?.token;
        let rt = self.reference_token(&user.id).await?.token;
        Ok(AuthLoginRes { at, rt })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn logout(&self, req: &AuthLogoutReq, sub: &str) -> Result<AuthLogoutRes, Status> {
        self.logout_all(sub).await
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn token(&self, req: &AuthTokenReq, sub: &str) -> Result<AuthTokenRes, Status> {
        let _rt = self
            .reference_token_service
            .validate(
                &RtValidateReq {
                    claims_jti: req.token.clone(),
                },
                sub,
            )
            .await?;
        self.access_token(sub).await
    }
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    fn hash(&self, msg: String) -> String {
        msg
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    async fn access_token(&self, sub: &str) -> Result<AuthTokenRes, Status> {
        let iat = Utc::now().timestamp();
        let exp = iat + self.config.jwt_at_exp_in();
        let jti = Uuid::new_v4().to_string().to_uppercase();
        let nbf = iat + self.config.jwt_at_nbf_in();
        let jwks = self
            .jwks_service
            .find_one_random(&JwksFindOneRandomReq {}, sub)
            .await?;
        let header = Header {
            typ: Some("JWT".into()),
            alg: Algorithm::RS256,
            cty: None,
            jku: None,
            kid: Some(jwks.id),
            x5u: None,
            x5t: None,
        };
        let claims = AuthClaims {
            aud: self.config.app_name(),
            exp,
            iat,
            iss: self.config.app_name(),
            jti,
            nbf,
            sub: sub.into(),
        };
        let key = EncodingKey::from_rsa_pem(jwks.private_key.as_bytes()).map_err(|err| {
            tracing::error!("{:?}", err);
            Error::from(err)
        })?;
        let token = jsonwebtoken::encode(&header, &claims, &key).map_err(|err| {
            tracing::error!("{:?}", err);
            Error::from(err)
        })?;
        let at = AtSaveReq {
            header_typ: header.typ.unwrap_or_default(),
            header_alg: "RS256".into(),
            header_cty: header.cty.unwrap_or_default(),
            header_jku: header.jku.unwrap_or_default(),
            header_kid: header.kid.unwrap_or_default(),
            header_x5u: header.x5u.unwrap_or_default(),
            header_x5t: header.x5t.unwrap_or_default(),
            claims_aud: claims.aud,
            claims_exp: claims.exp,
            claims_iat: claims.iat,
            claims_iss: claims.iss,
            claims_jti: claims.jti,
            claims_nbf: claims.nbf,
            claims_sub: claims.sub,
            token_blocked: false,
            token_blocked_description: "".into(),
        };
        let _rows_affected = self.access_token_service.save(&at, sub).await?;
        Ok(AuthTokenRes { token })
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    async fn logout_all(&self, sub: &str) -> Result<AuthLogoutRes, Status> {
        // self.at_service.update(&AtUpdateReq {}, sub).await?;
        // self.rt_service.update(&RtUpdateReq {}, sub).await?;
        Ok(AuthLogoutRes {})
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    async fn reference_token(&self, sub: &str) -> Result<AuthTokenRes, Status> {
        let iat = Utc::now().timestamp();
        let exp = iat + self.config.jwt_rt_exp_in();
        let jti = Uuid::new_v4().to_string().to_uppercase();
        let nbf = iat + self.config.jwt_rt_nbf_in();
        let rt = RtSaveReq {
            claims_aud: self.config.app_name(),
            claims_exp: exp,
            claims_iat: iat,
            claims_iss: self.config.app_name(),
            claims_jti: jti.clone(),
            claims_nbf: nbf,
            claims_sub: sub.into(),
            token_blocked: false,
            token_blocked_description: "".into(),
        };
        let _rows_affected = self.reference_token_service.save(&rt, sub).await?;
        Ok(AuthTokenRes { token: jti })
    }
}
