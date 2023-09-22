use crate::session::store::{SessionDataValueStorage, SessionStore};
use crate::session::SessionKey;
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
    S: CreateNew,
    K: SessionKey + Clone + Eq + Hash + Send + Sync,
{
    type Session = Arc<S>;
    type Key = K;

    async fn create_session(&self) -> anyhow::Result<(Self::Key, Self::Session)> {
        let key = K::generate();
        let session = Arc::new(S::new());

        self.sessions.lock().insert(key.clone(), session.clone());

        Ok((key, session))
    }

    async fn load_session(&self, key: &K) -> anyhow::Result<Option<Self::Session>> {
        Ok(self.sessions.lock().get(key).cloned())
    }
}

pub trait CreateNew: Send + Sync {
    fn new() -> Self;
}

impl<S> CreateNew for Arc<S>
where
    S: CreateNew,
{
    fn new() -> Self {
        Arc::new(S::new())
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

impl<K, V> CreateNew for InMemorySessionData<K, V>
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
    K: Clone + Hash + Eq + Send,
    V: Clone + Send,
{
    async fn set_value<KVal, VVal>(&self, key: KVal, val: VVal) -> anyhow::Result<()>
    where
        KVal: Into<K> + Send,
        VVal: Into<V> + Send,
    {
        self.values.lock().insert(key.into(), val.into());
        Ok(())
    }

    async fn unset_value<KVal>(&self, key: KVal) -> anyhow::Result<()>
    where
        KVal: Into<K> + Send,
    {
        self.values.lock().remove(&key.into());
        Ok(())
    }

    async fn get_value<KRef>(&self, key: &KRef) -> anyhow::Result<Option<V>>
    where
        KRef: Hash + Eq + ?Sized + Sync,
        K: Borrow<KRef>,
    {
        Ok(self.values.lock().get(key).cloned())
    }
}
