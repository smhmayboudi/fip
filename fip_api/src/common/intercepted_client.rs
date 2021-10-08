#![allow(clippy::type_complexity)]

//header::HeaderValue,
use futures::TryFutureExt;
use http::{Request, Response};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tonic::{body::BoxBody, transport::Body, transport::Channel};
use tower_service::Service;
// use uuid::Uuid;

/// TODO: documentation
#[derive(Debug)]
pub struct InterceptedClient {
    inner: Channel,
}

impl InterceptedClient {
    /// TODO: documentation
    pub fn new(inner: Channel) -> Self {
        Self { inner }
    }
}

impl Service<Request<BoxBody>> for InterceptedClient {
    type Response = Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        let inner = self.inner.clone();
        let mut svc = std::mem::replace(&mut self.inner, inner);
        let fut = async move {
            // let headers = req.headers_mut();
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
            svc.call(req).err_into::<Self::Error>().await
        };
        Box::pin(fut)
    }
}
