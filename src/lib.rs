use tower::Layer;

#[derive(Clone, Copy)]
pub struct HttpLogLayer;

#[derive(Clone, Copy)]
pub struct LogService<S> {
    inner: S,
}
impl HttpLogLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for HttpLogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}

type Req = hyper::Request<hyper::body::Incoming>;

impl<S> tower::Service<Req> for LogService<S>
where
    S: tower::Service<Req> + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: hyper::Request<hyper::body::Incoming>) -> Self::Future {
        let (parts, body) = request.into_parts();
        dbg!(&parts);
        self.inner.call(hyper::Request::from_parts(parts, body))
    }
}
