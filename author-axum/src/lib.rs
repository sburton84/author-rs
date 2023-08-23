use author_web::store::SessionStore;
use author_web::{Session as AuthorSession, SessionError};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{async_trait, RequestPartsExt};
use axum_extra::extract::cookie::{Cookie, Key, SameSite};
use axum_extra::extract::PrivateCookieJar;
use futures::future::BoxFuture;
use parking_lot::Mutex;
use std::convert::Infallible;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use thiserror::Error;
use tower_layer::Layer;
use tower_service::Service;
use tower_util::ServiceExt;
use tracing::error;
use uuid::Uuid;

pub use author_web::store;
pub use author_web::SessionConfig;

#[derive(Clone)]
pub struct Session(pub AuthorSession);

#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Session>()
            .cloned()
            .ok_or((StatusCode::FORBIDDEN, "Forbidden"))
    }
}

#[derive(Clone)]
pub struct SessionManagerService<S> {
    inner: S,
    config: SessionConfig,
    store: Arc<Mutex<dyn SessionStore>>,
}

impl<S> SessionManagerService<S> {
    pub fn new(inner: S, config: SessionConfig, store: Arc<Mutex<dyn SessionStore>>) -> Self {
        SessionManagerService {
            inner,
            config: config.into(),
            store,
        }
    }
}

impl<S, B, ResBody> Service<Request<B>> for SessionManagerService<S>
where
    S: Service<Request<B>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Response: IntoResponse,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = (Option<PrivateCookieJar>, Result<S::Response, StatusCode>);
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let config = self.config.clone();
        let mut store = self.store.clone();

        let clone = self.inner.clone();
        let inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let (mut parts, body) = req.into_parts();

            let mut cookie_jar = match parts
                .extract_with_state::<PrivateCookieJar, Key>(&config.key)
                .await
            {
                Err(e) => {
                    error!("Failed to extract session cookie: {}", e);
                    return Ok((None, Err(StatusCode::INTERNAL_SERVER_ERROR)));
                }
                Ok(j) => j,
            };

            let cookie = cookie_jar.get(&config.cookie_name);

            let session = match cookie {
                Some(c) => {
                    let uuid = match Uuid::from_str(c.value()) {
                        Err(e) => {
                            error!("Invalid UUID in session cookie: {}", e);
                            return Ok((None, Err(StatusCode::INTERNAL_SERVER_ERROR)));
                        }
                        Ok(u) => u,
                    };

                    // TODO: Refresh the session cookie with a new UUID

                    let session = match store.load_session(&uuid) {
                        None => {
                            error!("Session with key {} not found", uuid);
                            return Ok((None, Err(StatusCode::FORBIDDEN)));
                        }
                        Some(s) => s,
                    };

                    Session(session)
                }
                None => {
                    let session = AuthorSession::new();
                    let uuid = store.store_session(&session);

                    let cookie = Cookie::build(config.cookie_name.to_string(), uuid.to_string())
                        .same_site(SameSite::Strict)
                        .secure(true)
                        .http_only(true)
                        .finish();

                    cookie_jar = cookie_jar.add(cookie);

                    Session(session)
                }
            };

            parts.extensions.insert(session);

            let response = inner.oneshot(Request::from_parts(parts, body)).await?;

            Ok((Some(cookie_jar), Ok(response)))
        })
    }
}

#[derive(Clone)]
pub struct SessionManagerLayer {
    config: SessionConfig,
    store: Arc<Mutex<dyn SessionStore>>,
}

impl SessionManagerLayer {
    pub fn new(config: SessionConfig, store: Arc<Mutex<dyn SessionStore>>) -> Self {
        SessionManagerLayer { config, store }
    }
}

impl<S> Layer<S> for SessionManagerLayer {
    type Service = SessionManagerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionManagerService::new(inner, self.config.clone(), self.store.clone())
    }
}

#[derive(Debug, Error)]
pub enum AxumSessionError<E>
where
    E: IntoResponse,
{
    #[error("Error from inner service: {0}")]
    InnerServiceError(E),
    #[error("Unexpected session error: {0}")]
    SessionError(#[from] SessionError),
    #[error("Session store not found")]
    SessionStoreNotFound,
    #[error("Session config not found")]
    SessionConfigNotFound,
    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),
}

impl<E> IntoResponse for AxumSessionError<E>
where
    E: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            AxumSessionError::InnerServiceError(inner) => inner.into_response(),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
        }
    }
}
