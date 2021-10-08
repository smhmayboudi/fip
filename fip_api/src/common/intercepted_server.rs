use crate::{
    at::{config::Config as AtConfig, proto::client::AtValidateReq, service::Service as AtService},
    auth::{auth_claims::AuthClaims, auth_enforce::AuthEnforce},
    jwks::{
        config::Config as JwksConfig, proto::client::JwksFindOneReq,
        service::Service as JwksService,
    },
};
use futures::future::BoxFuture;
use futures::TryFutureExt;
use http::header::HeaderValue;
use hyper::{Body, Request, Response};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::NamedService, Status};
use tower_service::Service;
use uuid::Uuid;

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct InterceptedServer<S> {
    inner: S,
}

impl<S> InterceptedServer<S> {
    /// TODO: documentation
    pub const fn new(inner: S) -> Self {
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
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    /// TODO: documentation
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    /// TODO: documentation
    ///
    /// # Errors
    /// TODO: documentation errors
    ///
    /// # Panics
    /// TODO: documentation panics
    #[allow(clippy::too_many_lines)]
    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let fut = async move {
            if req.uri().path() == "/fip.api.auth.Auth/Login" {
                let headers = req.headers_mut();
                let uuidv4 = Uuid::new_v4().to_string().to_uppercase();
                if let Ok(header) = HeaderValue::from_str(&uuidv4).map_err(|err| {
                    tracing::error!("{:?}", err);
                    // Status::unknown("")
                }) {
                    let _request_id = headers.insert("request_id", header);
                    inner.call(req).err_into::<Self::Error>().await
                } else {
                    Ok(Status::unauthenticated("").to_http())
                }
            } else if let Some(header_authorization) = req.headers().get("authorization") {
                if let Ok(header_authorization) = header_authorization.to_str().map_err(|err| {
                    tracing::error!("{:?}", err);
                    // Status::unknown("")
                }) {
                    let mut header_authorization_splits =
                        header_authorization.split_ascii_whitespace();
                    if let Some(token_schema) = header_authorization_splits.next() {
                        if token_schema.to_uppercase() == "JWT" {
                            if let Some(token_jwt) = header_authorization_splits.next() {
                                if let Ok(header) =
                                    jsonwebtoken::decode_header(token_jwt).map_err(|err| {
                                        tracing::error!("{:?}", err);
                                        // Status::unknown("")
                                    })
                                {
                                    if let Some(kid) = header.kid {
                                        let jwks_service = JwksService::new(JwksConfig::new());
                                        if let Ok(jwks) = jwks_service
                                            .find_one(
                                                &JwksFindOneReq { id: kid },
                                                "00000000-0000-0000-0000-000000000000",
                                            )
                                            .await
                                        {
                                            if let Ok(key) = DecodingKey::from_rsa_pem(
                                                jwks.public_key.as_bytes(),
                                            )
                                            .map_err(|err| {
                                                tracing::error!("{:?}", err);
                                                // Status::unknown("")
                                            }) {
                                                let validation = Validation::new(Algorithm::RS256);
                                                if let Ok(token_data) =
                                                    jsonwebtoken::decode::<AuthClaims>(
                                                        token_jwt,
                                                        &key,
                                                        &validation,
                                                    )
                                                    .map_err(|err| {
                                                        tracing::error!("{:?}", err);
                                                        // Status::unknown("")
                                                    })
                                                {
                                                    let at_service =
                                                        AtService::new(AtConfig::new());
                                                    if let Ok(_at) = at_service
                                                        .validate(
                                                            &AtValidateReq {
                                                                claims_jti: token_data
                                                                    .claims
                                                                    .jti
                                                                    .clone(),
                                                            },
                                                            &token_data.claims.sub,
                                                        )
                                                        .await
                                                    {
                                                        let sub = token_data.claims.sub;
                                                        let obj = req.uri().path();
                                                        let act = req.method().to_string();
                                                        let b =
                                                            AuthEnforce::enforce(&sub, obj, &act)
                                                                .await;
                                                        if b {
                                                            let headers = req.headers_mut();
                                                            // let uuidv4 = Uuid::new_v4()
                                                            //     .to_string()
                                                            //     .to_uppercase();
                                                            // let header =
                                                            //     HeaderValue::from_str(&uuidv4)
                                                            //         .map_err(|err| {
                                                            //             tracing::error!(
                                                            //                 "{:?}", err
                                                            //             );
                                                            //             // Status::unknown("")
                                                            //         })
                                                            //         .unwrap_or_else(|err| {
                                                            //             panic!("{:?}", err);
                                                            //         });
                                                            // let _request_id = headers
                                                            //     .insert("request_id", header);
                                                            if let Ok(header) =
                                                                HeaderValue::from_str(&sub).map_err(
                                                                    |err| {
                                                                        tracing::error!(
                                                                            "{:?}", err
                                                                        );
                                                                        // Status::unknown("")
                                                                    },
                                                                )
                                                            {
                                                                let _sub =
                                                                    headers.insert("sub", header);
                                                                inner
                                                                    .call(req)
                                                                    .err_into::<Self::Error>()
                                                                    .await
                                                            } else {
                                                                Ok(Status::unauthenticated("")
                                                                    .to_http())
                                                            }
                                                        } else {
                                                            Ok(Status::unauthenticated("")
                                                                .to_http())
                                                        }
                                                    } else {
                                                        Ok(Status::unauthenticated("").to_http())
                                                    }
                                                } else {
                                                    Ok(Status::unauthenticated("").to_http())
                                                }
                                            } else {
                                                Ok(Status::unauthenticated("").to_http())
                                            }
                                        } else {
                                            Ok(Status::unauthenticated("").to_http())
                                        }
                                    } else {
                                        Ok(Status::unauthenticated("").to_http())
                                    }
                                } else {
                                    Ok(Status::unauthenticated("").to_http())
                                }
                            } else {
                                Ok(Status::unauthenticated("").to_http())
                            }
                        } else {
                            Ok(Status::unauthenticated("").to_http())
                        }
                    } else {
                        Ok(Status::unauthenticated("").to_http())
                    }
                } else {
                    Ok(Status::unauthenticated("").to_http())
                }
            } else {
                Ok(Status::unauthenticated("").to_http())
            }
        };
        Box::pin(fut)
    }
}

impl<S: NamedService> NamedService for InterceptedServer<S> {
    const NAME: &'static str = S::NAME;
}
