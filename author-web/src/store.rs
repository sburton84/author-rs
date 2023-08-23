use crate::Session;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use uuid::Uuid;

pub trait Key {
    fn generate() -> Self;
}

impl Key for Uuid {
    fn generate() -> Self {
        Uuid::new_v4()
    }
}

pub trait SessionStore<S = Session, K = Uuid>: Send {
    fn store_session(&mut self, session: &S) -> K;
    fn load_session(&self, key: &K) -> Option<S>;
}

pub struct InMemorySessionStore<S, K> {
    sessions: HashMap<K, S>,
}

impl<S, K> InMemorySessionStore<S, K> {
    pub fn new() -> Self {
        InMemorySessionStore {
            sessions: HashMap::new(),
        }
    }
}

impl<S, K> SessionStore<S, K> for InMemorySessionStore<S, K>
where
    S: Clone + Send + Sync,
    K: Key + Clone + Eq + Hash + Send + Sync,
{
    fn store_session(&mut self, session: &S) -> K {
        let key = K::generate();
        self.sessions.insert(key.clone(), session.clone());

        key
    }

    fn load_session(&self, key: &K) -> Option<S> {
        self.sessions.get(key).cloned()
    }
}

impl<S, K> SessionStore<S, K> for Arc<Mutex<dyn SessionStore<S, K>>> {
    fn store_session(&mut self, session: &S) -> K {
        self.lock().store_session(session)
    }

    fn load_session(&self, key: &K) -> Option<S> {
        self.lock().load_session(key)
    }
}
