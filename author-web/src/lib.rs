use cookie::Key;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

pub mod store;

pub trait SessionData: Send + Sync {
    fn new() -> Self;
}

#[derive(Clone)]
pub struct InMemorySession {
    values: HashMap<String, String>,
}

impl InMemorySession {
    pub fn new() -> Self {
        InMemorySession {
            values: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, key: &str, val: &str) {
        self.values.insert(key.to_string(), val.to_string());
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }
}

impl SessionData for InMemorySession {
    fn new() -> Self {
        InMemorySession::new()
    }
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
