use cookie::Key;
use std::sync::Arc;
use thiserror::Error;

pub mod session;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session with given ID not found")]
    SessionNotFound,
    #[error("Unexpected session error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Clone)]
pub struct SessionConfig {
    pub cookie_name: Arc<str>,
    pub key: Key,
}

impl SessionConfig {
    pub fn new(cookie_name: impl AsRef<str>, key: Key) -> Self {
        SessionConfig {
            cookie_name: cookie_name.as_ref().into(),
            key,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            cookie_name: "author_session_cookie".into(),
            key: Key::generate(),
        }
    }
}
