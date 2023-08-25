use crate::session::store::SessionStore;
use crate::session::{SessionData, SessionDataValues, SessionKey};
use parking_lot::Mutex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use uuid::Uuid;

pub struct InMemorySessionStore<S = InMemorySessionData<String, String>, K = Uuid> {
    sessions: HashMap<K, Arc<Mutex<S>>>,
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
    type Session = Arc<Mutex<S>>;
    type Key = K;

    fn create_session(&mut self) -> (Self::Key, Self::Session) {
        let key = K::generate();
        let session = Arc::new(Mutex::new(S::new()));

        self.sessions.insert(key.clone(), session.clone());

        (key, session)
    }

    fn load_session(&self, key: &K) -> Option<Self::Session> {
        self.sessions.get(key).cloned()
    }
}

pub type InMemorySession<K = String, V = String> = Arc<Mutex<InMemorySessionData<K, V>>>;

#[derive(Clone)]
pub struct InMemorySessionData<K = String, V = String> {
    values: HashMap<K, V>,
}

impl<K, V> InMemorySessionData<K, V> {
    pub fn new() -> Self {
        InMemorySessionData {
            values: HashMap::new(),
        }
    }
}

impl<K, V> SessionData for InMemorySessionData<K, V>
where
    K: Send + Sync,
    V: Send + Sync,
{
    fn new() -> Self {
        InMemorySessionData::new()
    }
}

impl SessionKey for Uuid {
    fn generate() -> Self {
        Uuid::new_v4()
    }
}

impl<K, V> SessionDataValues<K, V> for InMemorySessionData<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn set_value(&mut self, key: K, val: V) {
        self.values.insert(key, val);
    }

    fn get_value<KRef>(&self, key: &KRef) -> Option<V>
    where
        KRef: Hash + Eq + ?Sized,
        K: Borrow<KRef>,
    {
        self.values.get(key).cloned()
    }
}
