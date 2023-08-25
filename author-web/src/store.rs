use crate::{InMemorySession, SessionData};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub trait SessionKey: FromStr {
    fn generate() -> Self;
}

impl SessionKey for Uuid {
    fn generate() -> Self {
        Uuid::new_v4()
    }
}

pub trait SessionStore: Send {
    type Session: SessionData;
    type Key: SessionKey;

    fn store_session(&mut self, session: &Self::Session) -> Self::Key;
    fn load_session(&self, key: &Self::Key) -> Option<Self::Session>;
}

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

impl<S, K> SessionStore for Arc<Mutex<dyn SessionStore<Session = S, Key = K>>>
where
    S: SessionData,
    K: SessionKey,
{
    type Session = S;
    type Key = K;

    fn store_session(&mut self, session: &S) -> K {
        self.lock().store_session(session)
    }

    fn load_session(&self, key: &K) -> Option<S> {
        self.lock().load_session(key)
    }
}

impl<S, K, Store> SessionStore for Arc<Mutex<Store>>
where
    Store: SessionStore<Session = S, Key = K>,
    S: SessionData,
    K: SessionKey,
{
    type Session = S;
    type Key = K;

    fn store_session(&mut self, session: &S) -> K {
        self.lock().store_session(session)
    }

    fn load_session(&self, key: &K) -> Option<S> {
        self.lock().load_session(key)
    }
}
