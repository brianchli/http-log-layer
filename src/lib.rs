use tower::Layer;

#[derive(Clone, Copy)]
pub struct HttpLogLayer {
    config: Config,
}

#[derive(Clone, Copy)]
pub struct LogService<S> {
    inner: S,
}

#[derive(Clone, Copy)]
pub struct Config {}

impl Config {
    pub fn builder() -> Self {
        Self {}
    }
}

impl HttpLogLayer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl<S> Layer<S> for HttpLogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}

type Req<Body> = hyper::Request<Body>;
impl<S, B> tower::Service<Req<B>> for LogService<S>
where
    S: tower::Service<Req<B>> + Clone,
    B: hyper::body::Body,
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

    fn call(&mut self, request: Req<B>) -> Self::Future {
        let (parts, body) = request.into_parts();
        println!("access by {:?}", &parts.headers.get("x-client-ip").unwrap());
        self.inner.call(hyper::Request::from_parts(parts, body))
    }
}
