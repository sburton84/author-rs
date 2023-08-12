use cookie::Key;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Session {
    pub uuid: Uuid,
}

impl Session {
    pub fn new() -> Self {
        Session {
            uuid: Uuid::new_v4(),
        }
    }
}

pub trait SessionStore: Send + Sync {
    fn store_session(&self, session: &Session);
    fn load_session(&self, uuid: Uuid) -> Session;
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

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session with given ID not found")]
    SessionNotFound,
    #[error("Unexpected session error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}
