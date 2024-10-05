use crate::session::SessionKey;
use async_trait::async_trait;
use std::borrow::Borrow;
use std::hash::Hash;

#[cfg(feature = "in-memory")]
pub mod in_memory;

#[async_trait]
pub trait SessionStore: Send {
    type Session;
    type Key: SessionKey;

    async fn create_session(&self) -> anyhow::Result<(Self::Key, Self::Session)>;
    async fn load_session(&self, key: &Self::Key) -> anyhow::Result<Option<Self::Session>>;
}

#[async_trait]
pub trait SessionDataValueStorage<K, V>
where
    K: Hash + Eq,
{
    async fn set_value<KVal, VVal>(&self, key: KVal, val: VVal) -> anyhow::Result<()>
    where
        KVal: Into<K> + Send,
        VVal: Into<V> + Send;

    async fn unset_value<KVal>(&self, key: KVal) -> anyhow::Result<()>
    where
        KVal: Into<K> + Send;

    async fn get_value<KRef>(&self, key: &KRef) -> anyhow::Result<Option<V>>
    where
        KRef: Hash + Eq + ?Sized + ToOwned<Owned = K> + Sync,
        K: Borrow<KRef> + Hash + Eq;
}
