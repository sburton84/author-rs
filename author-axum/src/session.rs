use author_web::session::store::in_memory::InMemorySessionData;
use author_web::session::store::SessionStore;
use author_web::session::{SessionConfig, SessionError, SessionKey};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{async_trait, RequestPartsExt};
use axum_extra::extract::cookie::{Cookie, Key};
use axum_extra::extract::PrivateCookieJar;
use futures::future::BoxFuture;
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use thiserror::Error;
use tower_layer::Layer;
use tower_service::Service;
use tower_util::ServiceExt;
use tracing::{debug, error, trace};

#[derive(Clone)]
pub struct Session<T: Clone = Arc<InMemorySessionData>>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Session<T>
where
    S: Send + Sync,
    T: Clone + Send + Sync + 'static,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Session<T>>()
            .cloned()
            .ok_or((StatusCode::FORBIDDEN, "Forbidden"))
    }
}

pub struct SessionManagerService<Inner, Store>
where
    Store: SessionStore,
{
    inner: Inner,
    config: SessionConfig,
    store: Arc<Store>,
}

impl<Inner, Store> SessionManagerService<Inner, Store>
where
    Store: SessionStore,
{
    pub fn new(inner: Inner, config: SessionConfig, store: Arc<Store>) -> Self {
        SessionManagerService {
            inner,
            config: config.into(),
            store,
        }
    }
}

// #[derive(Clone)] requires Store to be Clone, which shouldn't really be necessary because it's
// in an Arc. The only way to get around this is to manually implement Clone.
// See https://github.com/rust-lang/rust/issues/26925
impl<Inner, Store> Clone for SessionManagerService<Inner, Store>
where
    Inner: Clone,
    Store: SessionStore,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            config: self.config.clone(),
            store: self.store.clone(),
        }
    }
}

impl<Inner, S, K, B, ResBody, Store> Service<Request<B>> for SessionManagerService<Inner, Store>
where
    Inner: Service<Request<B>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    Inner::Response: IntoResponse,
    Inner::Future: Send,
    B: Send + 'static,
    K: SessionKey + Display + Send + Sync + 'static,
    <K as FromStr>::Err: Send,
    S: Clone + Send + Sync + 'static,
    Store: SessionStore<Session = S, Key = K> + Send + Sync + 'static,
{
    type Response = (
        Option<PrivateCookieJar>,
        Result<Inner::Response, StatusCode>,
    );
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let config = self.config.clone();
        let store = self.store.clone();

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

            // Check whether we have any existing session
            let existing_session = match cookie {
                Some(c) => {
                    let session = match K::from_str(c.value()) {
                        Err(_) => {
                            error!("Error parsing key in session cookie: {}", c.value());
                            None
                        }
                        Ok(session_key) => {
                            debug!(
                                "Existing session cookie found containing key {}",
                                session_key
                            );

                            // TODO: Refresh the session cookie with a new key

                            match store.load_session(&session_key).await {
                                Err(e) => {
                                    error!("Failed to load session: {}", e);
                                    None
                                }
                                Ok(u) => match u {
                                    None => {
                                        error!("Session with key {} not found", session_key);
                                        None
                                    }
                                    Some(s) => Some(s),
                                },
                            }
                        }
                    };

                    session
                }
                None => None,
            };

            // If there's no usable existing session for any reason, create a new one
            let session = match existing_session {
                Some(s) => s,
                None => {
                    debug!("No existing session found, creating new session");

                    let (session_key, session) = match store.create_session().await {
                        Err(e) => {
                            error!("Failed to create session: {}", e);
                            return Ok((None, Err(StatusCode::INTERNAL_SERVER_ERROR)));
                        }
                        Ok(s) => s,
                    };

                    trace!("Session created with key {}", session_key);

                    let cookie =
                        Cookie::build((config.cookie_name.to_string(), session_key.to_string()))
                            .same_site(config.same_site)
                            .secure(true)
                            .http_only(true)
                            .path("/")
                            .build();

                    cookie_jar = cookie_jar.add(cookie);

                    session
                }
            };

            trace!("Adding session to extensions");

            parts.extensions.insert(Session(session));

            trace!("Processing inner service");

            let response = inner.oneshot(Request::from_parts(parts, body)).await?;

            Ok((Some(cookie_jar), Ok(response)))
        })
    }
}

pub struct SessionManagerLayer<Store>
where
    Store: SessionStore,
{
    config: SessionConfig,
    store: Arc<Store>,
}

// #[derive(Clone)] requires Store to be Clone, which shouldn't really be necessary because it's
// in an Arc. The only way to get around this is to manually implement Clone.
// See https://github.com/rust-lang/rust/issues/26925
impl<Store> Clone for SessionManagerLayer<Store>
where
    Store: SessionStore,
{
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            store: self.store.clone(),
        }
    }
}

impl<Store> SessionManagerLayer<Store>
where
    Store: SessionStore,
{
    pub fn new(config: SessionConfig, store: Store) -> Self {
        SessionManagerLayer {
            config,
            store: Arc::new(store),
        }
    }
}

impl<Inner, Store> Layer<Inner> for SessionManagerLayer<Store>
where
    Store: SessionStore,
{
    type Service = SessionManagerService<Inner, Store>;

    fn layer(&self, inner: Inner) -> Self::Service {
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
            AxumSessionError::SessionError(SessionError::SessionNotFound) => {
                (StatusCode::FORBIDDEN, "Forbidden").into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
        }
    }
}
