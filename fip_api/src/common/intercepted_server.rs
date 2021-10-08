use crate::{
    at::{
        config::Config as AtConfig,
        proto::client::{AtRes, AtValidateReq},
        service::Service as AtService,
    },
    auth::auth_claims::AuthClaims,
    auth::auth_enforce::AuthEnforce,
    jwks::{
        config::Config as JwksConfig,
        proto::client::{JwksFindOneReq, JwksRes},
        service::Service as JwksService,
    },
};
use futures::TryFutureExt;
use http::header::HeaderValue;
use hyper::{Body, Request, Response};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService};
use tower_service::Service;
use uuid::Uuid;

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct InterceptedServer<S> {
    inner: S,
}

impl<S> InterceptedServer<S> {
    /// TODO: documentation
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> Service<Request<Body>> for InterceptedServer<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + NamedService + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let inner = self.inner.clone();
        let mut svc = std::mem::replace(&mut self.inner, inner);
        let fut = async move {
            if req.uri().path() == "/fip.api.auth.Auth/Login" {
                let headers = req.headers_mut();
                let uuidv4 = Uuid::new_v4().to_string().to_uppercase();
                let _ = headers.insert(
                    "request_id",
                    HeaderValue::from_str(&uuidv4)
                        .map_err(|err| {
                            tracing::error!("{:?}", err);
                            // Status::unknown("")
                        })
                        .unwrap(),
                );

                svc.call(req).err_into::<Self::Error>().await
            } else if let Some(token) = req.headers().get("authorization") {
                let token = token
                    .to_str()
                    .map_err(|err| {
                        tracing::error!("{:?}", err);
                        // Status::unknown("")
                    })
                    .unwrap();

                let mut tokens = token.split_ascii_whitespace();

                let token = tokens.next().unwrap();
                if token.to_uppercase() == "JWT" {
                    let token = tokens.next().unwrap();
                    let header = jsonwebtoken::decode_header(token)
                        .map_err(|err| {
                            tracing::error!("{:?}", err);
                            // Status::unknown("")
                        })
                        .unwrap();
                    let kid = header.kid.unwrap();

                    let jwks_service = JwksService::new(JwksConfig::new());
                    let jwks = jwks_service
                        .find_one(
                            &JwksFindOneReq { id: kid },
                            "00000000-0000-0000-0000-000000000000",
                        )
                        .await
                        .unwrap_or_default();
                    if jwks != JwksRes::default() {
                        let key = DecodingKey::from_rsa_pem(&jwks.public_key.as_bytes())
                            .map_err(|err| {
                                tracing::error!("{:?}", err);
                                // Status::unknown("")
                            })
                            .unwrap();
                        let validation = Validation::new(Algorithm::RS256);
                        let token_data =
                            jsonwebtoken::decode::<AuthClaims>(&token, &key, &validation)
                                .map_err(|err| {
                                    tracing::error!("{:?}", err);
                                    // Status::unknown("")
                                })
                                .unwrap();

                        let at_service = AtService::new(AtConfig::new());
                        let at = at_service
                            .validate(
                                &AtValidateReq {
                                    claims_jti: token_data.claims.jti.clone(),
                                },
                                &token_data.claims.sub,
                            )
                            .await
                            .unwrap_or_default();
                        if at != AtRes::default() {
                            let sub = token_data.claims.sub;
                            let obj = req.uri().path();
                            let act = req.method().to_string();
                            let b = AuthEnforce::enforce(&sub, obj, &act).await;
                            if b {
                                let headers = req.headers_mut();
                                // let uuidv4 = Uuid::new_v4().to_string().to_uppercase();
                                // let _ = headers.insert(
                                //     "request_id",
                                //     HeaderValue::from_str(&uuidv4)
                                //         .map_err(|err| {
                                //             tracing::error!("{:?}", err);
                                //             // Status::unknown("")
                                //         })
                                //         .unwrap(),
                                // );
                                let _ = headers.insert(
                                    "sub",
                                    HeaderValue::from_str(&sub)
                                        .map_err(|err| {
                                            tracing::error!("{:?}", err);
                                            // Status::unknown("")
                                        })
                                        .unwrap(),
                                );

                                svc.call(req).err_into::<Self::Error>().await
                            } else {
                                Ok(tonic::Status::unauthenticated("").to_http())
                            }
                        } else {
                            Ok(tonic::Status::unauthenticated("").to_http())
                        }
                    } else {
                        Ok(tonic::Status::unauthenticated("").to_http())
                    }
                } else {
                    Ok(tonic::Status::unauthenticated("").to_http())
                }
            } else {
                Ok(tonic::Status::unauthenticated("").to_http())
            }
        };
        Box::pin(fut)
    }
}

impl<S: NamedService> NamedService for InterceptedServer<S> {
    const NAME: &'static str = S::NAME;
}
