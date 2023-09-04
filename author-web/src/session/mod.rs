use cookie::Key;
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

pub trait SessionKey: FromStr {
    fn generate() -> Self;
}

pub trait SessionData: Send + Sync {
    fn new() -> Self;
}

pub trait SessionSubject<Subject> {
    fn subject() -> Subject;
}

impl<S> SessionData for Arc<S>
where
    S: SessionData,
{
    fn new() -> Self {
        Arc::new(S::new())
    }
}

// impl<S> SessionData for Arc<Mutex<S>>
// where
//     S: SessionData,
// {
//     fn new() -> Self {
//         Arc::new(Mutex::new(S::new()))
//     }
// }
