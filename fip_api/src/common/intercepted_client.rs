use futures::TryFutureExt;
// use http::header::HeaderValue;
use futures::future::BoxFuture;
use http::{Request, Response};
use std::task::{Context, Poll};
use tonic::{body::BoxBody, transport::Body, transport::Channel};
use tower_service::Service;
// use uuid::Uuid;

/// TODO: documentation
#[derive(Debug)]
pub struct InterceptedClient {
    inner: Channel,
}

/// TODO: documentation
impl InterceptedClient {
    /// TODO: documentation
    #[must_use]
    pub const fn new(inner: Channel) -> Self {
        Self { inner }
    }
}

/// TODO: documentation
impl Service<Request<BoxBody>> for InterceptedClient {
    type Response = Response<Body>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    /// TODO: documentation
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    /// TODO: documentation
    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        let inner = self.inner.clone();
        let mut svc = std::mem::replace(&mut self.inner, inner);
        let fut = async move {
            // let headers = req.headers_mut();
            // let uuidv4 = Uuid::new_v4().to_string().to_uppercase();
            // let header = HeaderValue::from_str(&uuidv4)
            //     .map_err(|err| {
            //         tracing::error!("{:?}", err);
            //         // Status::unknown("")
            //     })
            //     .unwrap_or_else(|err| {
            //         panic!("{:?}", err);
            //     });
            // let _request_id = headers.insert("request_id", header);
            svc.call(req).err_into::<Self::Error>().await
        };
        Box::pin(fut)
    }
}
