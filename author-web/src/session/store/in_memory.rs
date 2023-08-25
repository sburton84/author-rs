use crate::session::store::SessionStore;
use crate::session::{SessionData, SessionKey};
use std::collections::HashMap;
use std::hash::Hash;
use uuid::Uuid;

pub struct InMemorySessionStore<S = InMemorySession, K = Uuid> {
    sessions: HashMap<K, S>,
}

impl<S, K> InMemorySessionStore<S, K> {
    pub fn new() -> Self {
        InMemorySessionStore {
            sessions: HashMap::new(),
        }
    }
}

impl<S, K> SessionStore for InMemorySessionStore<S, K>
where
    S: SessionData + Clone + Send + Sync,
    K: SessionKey + Clone + Eq + Hash + Send + Sync,
{
    type Session = S;
    type Key = K;

    fn store_session(&mut self, session: &S) -> K {
        let key = K::generate();
        self.sessions.insert(key.clone(), session.clone());

        key
    }

    fn load_session(&self, key: &K) -> Option<S> {
        self.sessions.get(key).cloned()
    }
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

impl SessionKey for Uuid {
    fn generate() -> Self {
        Uuid::new_v4()
    }
}
