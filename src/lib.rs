mod config;
mod future;
pub use crate::config::Config;
use crate::future::LoggingFuture;

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use std::error::Error;
use std::fmt::Debug;
use tower::Layer;

#[derive(Clone, Copy)]
pub struct HttpLogLayer {}

#[derive(Clone, Copy)]
pub struct LogService<S> {
    inner: S,
}

impl<S> Layer<S> for HttpLogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}

impl HttpLogLayer {
    pub fn new(_config: Config) -> Self {
        tracing_subscriber::fmt().init();
        Self {}
    }
}

type Req<B> = hyper::Request<B>;
impl<S, B> tower::Service<Req<B>> for LogService<S>
where
    S: tower::Service<Req<B>>,
    S::Error: Into<Box<dyn Error + Send + Sync>> + Debug + 'static,
    S::Response: Debug
        + Into<hyper::Response<BoxBody<Bytes, S::Error>>>
        + From<hyper::Response<BoxBody<Bytes, S::Error>>>,
    B: hyper::body::Body,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LoggingFuture<S::Future, S::Response, S::Error>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Req<B>) -> Self::Future {
        LoggingFuture::new(self.inner.call(request))
    }
}
