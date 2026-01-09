use std::{fmt::Debug, pin::Pin};

use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use tracing::{Level, info, span};

pub struct LoggingFuture<F, B, E>
where
    F: Future<Output = Result<B, E>>,
{
    pub f: Pin<Box<F>>,
    span: tracing::span::Span,
}

impl<F, B, E> LoggingFuture<F, B, E>
where
    F: Future<Output = Result<B, E>>,
{
    pub fn new(f: F) -> Self {
        Self {
            f: Box::pin(f),
            span: span!(Level::INFO, "request"),
        }
    }
}

impl<F, B, E> Future for LoggingFuture<F, B, E>
where
    F: Future<Output = Result<B, E>>,
    F::Output: Debug,
    B: Into<hyper::Response<BoxBody<Bytes, E>>> + From<hyper::Response<BoxBody<Bytes, E>>>,
{
    type Output = F::Output;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let polled = self.f.as_mut().poll(cx);
        if let std::task::Poll::Ready(Ok(o)) = polled {
            let o: hyper::Response<BoxBody<Bytes, E>> = o.into();
            let (parts, body) = o.into_parts();
            info!(parent: &self.span, "{:?}", &parts);
            std::task::Poll::Ready(Ok(hyper::Response::from_parts(parts, body).into()))
        } else {
            polled
        }
    }
}
