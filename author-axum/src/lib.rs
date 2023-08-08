use author_web::Session as AuthorSession;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Response;
use std::future::Future;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;

pub struct Session(AuthorSession);
#[async_trait]
impl<S> FromRequestParts<S> for Session {
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}

pub struct SessionManagerService<S> {
    inner: S,
}

impl<S, Request> Service<Request> for SessionManagerService<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call(req)
    }
}

#[derive(Clone)]
pub struct SessionManagerLayer {}

impl SessionManagerLayer {
    pub fn new() -> Self {
        SessionManagerLayer {}
    }
}

impl<S> Layer<S> for SessionManagerLayer {
    type Service = SessionManagerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        todo!()
    }
}
