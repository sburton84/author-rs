use author_web::Session as AuthorSession;
use author_web::SessionStore;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::Response;
use axum::{async_trait, Extension, RequestPartsExt};
use axum_extra::extract::PrivateCookieJar;
use std::convert::Infallible;
use std::future::Future;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;

pub struct Session(AuthorSession);

struct SessionConfig {
    cookie_name: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            cookie_name: "author_session_cookie".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie = {
            let cookie_jar = PrivateCookieJar::from_request_parts(parts, state)
                .await
                .ok();

            let config = parts
                .extensions
                .get::<SessionConfig>()
                .unwrap_or(&SessionConfig::default());

            cookie_jar.map(|j| j.get(&config.cookie_name))
        };

        let session_store = parts.extensions.get::<Box<dyn SessionStore>>();
        // .await
        // .map_err(|err| err.into_response())?;

        let session = match cookie {
            Some(c) => {
                let id = c.value();
            }
            None => {}
        };

        Ok(session)
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
        SessionManagerService { inner }
    }
}
