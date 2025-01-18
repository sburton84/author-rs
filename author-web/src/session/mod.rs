use cookie::{Key, SameSite};
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;

pub mod store;

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
    pub same_site: SameSite,
    pub secure: bool,
}

impl SessionConfig {
    pub fn new(cookie_name: impl AsRef<str>, key: Key, same_site: SameSite, secure: bool) -> Self {
        SessionConfig {
            cookie_name: cookie_name.as_ref().into(),
            key,
            same_site,
            secure,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            cookie_name: "author_session_cookie".into(),
            key: Key::generate(),
            same_site: SameSite::Strict,
            secure: true,
        }
    }
}

pub trait SessionKey: FromStr {
    fn generate() -> Self;
}

pub trait SessionSubject<Subject> {
    fn subject() -> Subject;
}
