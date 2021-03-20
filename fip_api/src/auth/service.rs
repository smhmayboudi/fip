use crate::{
    at::{proto::client::AtSaveReqDto, service::Service as AtService},
    auth::{
        auth_claims::AuthClaims,
        config::Config,
        proto::server::{
            AuthLoginReqDto, AuthLoginResDto, AuthLogoutReqDto, AuthLogoutResDto, AuthTokenReqDto,
            AuthTokenResDto,
        },
    },
    jwks::{proto::client::JwksFindOneRandomReqDto, service::Service as JwksService},
    rt::{
        proto::client::{RtSaveReqDto, RtValidateReqDto},
        service::Service as RtService,
    },
    user::{proto::client::UserFindOneByUsernameReqDto, service::Service as UserService},
};
use chrono::Utc;
use fip_common::common_error::CommonError;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use tonic::Status;
use uuid::Uuid;

#[derive(Debug)]
pub struct Service {
    config: Config,
    at_service: AtService,
    jwks_service: JwksService,
    rt_service: RtService,
    user_service: UserService,
}

impl Service {
    pub fn new(
        config: Config,
        at_service: AtService,
        jwks_service: JwksService,
        rt_service: RtService,
        user_service: UserService,
    ) -> Self {
        Self {
            config,
            at_service,
            jwks_service,
            rt_service,
            user_service,
        }
    }
}

impl Service {
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn login(&self, req: &AuthLoginReqDto) -> Result<AuthLoginResDto, Status> {
        let user = self
            .user_service
            .find_one_by_username(
                &UserFindOneByUsernameReqDto {
                    username: req.username.clone(),
                },
                "00000000-0000-0000-0000-000000000000",
            )
            .await?;
        if self.hash(req.password.clone()) != user.password {
            panic!("password incorrect.")
        }
        let at = self.access_token(&user.id).await?.token;
        let rt = self.refresh_token(&user.id).await?.token;
        Ok(AuthLoginResDto { at, rt })
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn logout(
        &self,
        _req: &AuthLogoutReqDto,
        sub: &str,
    ) -> Result<AuthLogoutResDto, Status> {
        self.logout_all(sub).await
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn token(&self, req: &AuthTokenReqDto, sub: &str) -> Result<AuthTokenResDto, Status> {
        let _ = self
            .rt_service
            .validate(
                &RtValidateReqDto {
                    claims_jti: req.token.clone(),
                },
                sub,
            )
            .await?;
        self.access_token(sub).await
    }
}

impl Service {
    #[tracing::instrument(fields(otel.kind = "client"))]
    fn hash(&self, msg: String) -> String {
        msg
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    async fn access_token(&self, sub: &str) -> Result<AuthTokenResDto, Status> {
        let iat = Utc::now().timestamp();
        let exp = iat + self.config.jwt_at_exp_in();
        let jti = Uuid::new_v4().to_string().to_uppercase();
        let nbf = iat + self.config.jwt_at_nbf_in();
        let jwks = self
            .jwks_service
            .find_one_random(&JwksFindOneRandomReqDto {}, sub)
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
            exp: exp as usize,
            iat: iat as usize,
            iss: self.config.app_name(),
            jti: jti.clone(),
            nbf: nbf as usize,
            sub: sub.into(),
        };
        let key = EncodingKey::from_rsa_pem(jwks.private_key.as_bytes()).map_err(|err| {
            tracing::error!("{:?}", err);
            CommonError::from(err)
        })?;
        let token = jsonwebtoken::encode(&header, &claims, &key).map_err(|err| {
            tracing::error!("{:?}", err);
            CommonError::from(err)
        })?;
        let at = AtSaveReqDto {
            header_typ: header.typ.unwrap(),
            header_alg: "RS256".into(),
            header_cty: header.cty.unwrap_or_default(),
            header_jku: header.jku.unwrap_or_default(),
            header_kid: header.kid.unwrap_or_default(),
            header_x5u: header.x5u.unwrap_or_default(),
            header_x5t: header.x5t.unwrap_or_default(),
            claims_aud: claims.aud,
            claims_exp: claims.exp as i64,
            claims_iat: claims.iat as i64,
            claims_iss: claims.iss,
            claims_jti: claims.jti,
            claims_nbf: claims.nbf as i64,
            claims_sub: claims.sub,
            token_blocked: false,
            token_blocked_description: "".into(),
        };
        let _ = self.at_service.save(&at, sub).await?;
        Ok(AuthTokenResDto { token })
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    async fn logout_all(&self, _sub: &str) -> Result<AuthLogoutResDto, Status> {
        // self.at_service.update(&AtUpdateReqDto {}, sub).await?;
        // self.rt_service.update(&RtUpdateReqDto {}, sub).await?;
        Ok(AuthLogoutResDto {})
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    async fn refresh_token(&self, sub: &str) -> Result<AuthTokenResDto, Status> {
        let iat = Utc::now().timestamp();
        let exp = iat + self.config.jwt_rt_exp_in();
        let jti = Uuid::new_v4().to_string().to_uppercase();
        let nbf = iat + self.config.jwt_rt_nbf_in();
        let rt = RtSaveReqDto {
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
        let _ = self.rt_service.save(&rt, sub).await?;
        Ok(AuthTokenResDto { token: jti })
    }
}
