use crate::session::store::{SessionDataValueStorage, SessionStore};
use crate::session::{SessionData, SessionKey};
use async_trait::async_trait;
use parking_lot::Mutex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use uuid::Uuid;

pub struct InMemorySessionStore<S = InMemorySessionData<String, String>, K = Uuid> {
    sessions: Mutex<HashMap<K, Arc<S>>>,
}

impl<S, K> InMemorySessionStore<S, K> {
    pub fn new() -> Self {
        InMemorySessionStore {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl<S, K> SessionStore for InMemorySessionStore<S, K>
where
    S: SessionData,
    K: SessionKey + Clone + Eq + Hash + Send + Sync,
{
    type Session = Arc<S>;
    type Key = K;

    async fn create_session(&self) -> (Self::Key, Self::Session) {
        let key = K::generate();
        let session = Arc::new(S::new());

        self.sessions.lock().insert(key.clone(), session.clone());

        (key, session)
    }

    async fn load_session(&self, key: &K) -> Option<Self::Session> {
        self.sessions.lock().get(key).cloned()
    }
}

pub type InMemorySession<K = String, V = String> = Arc<InMemorySessionData<K, V>>;

pub struct InMemorySessionData<K = String, V = String> {
    values: Mutex<HashMap<K, V>>,
}

impl<K, V> InMemorySessionData<K, V> {
    pub fn new() -> Self {
        InMemorySessionData {
            values: Mutex::new(HashMap::new()),
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

#[async_trait]
impl<K, V> SessionDataValueStorage<K, V> for InMemorySessionData<K, V>
where
    K: Hash + Eq + Send,
    V: Clone + Send,
{
    async fn set_value<KVal, VVal>(&self, key: KVal, val: VVal)
    where
        KVal: Into<K> + Send,
        VVal: Into<V> + Send,
    {
        self.values.lock().insert(key.into(), val.into());
    }

    async fn get_value<KRef>(&self, key: &KRef) -> Option<V>
    where
        KRef: Hash + Eq + ?Sized + Sync,
        K: Borrow<KRef>,
    {
        self.values.lock().get(key).cloned()
    }
}
