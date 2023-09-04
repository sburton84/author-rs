use crate::session::{SessionData, SessionKey};
use async_trait::async_trait;
use std::borrow::Borrow;
use std::hash::Hash;

#[cfg(feature = "in-memory")]
pub mod in_memory;

#[async_trait]
pub trait SessionStore: Send {
    type Session: SessionData;
    type Key: SessionKey;

    async fn create_session(&self) -> (Self::Key, Self::Session);
    async fn load_session(&self, key: &Self::Key) -> Option<Self::Session>;
}

// #[async_trait]
// impl<S, K> SessionStore for Arc<Mutex<dyn SessionStore<Session = S, Key = K>>>
// where
//     S: SessionData,
//     K: SessionKey + Sync,
// {
//     type Session = S;
//     type Key = K;
//
//     async fn create_session(&mut self) -> (Self::Key, Self::Session) {
//         self.lock().await.create_session().await
//     }
//
//     async fn load_session(&self, key: &K) -> Option<S> {
//         let option = self.lock().await.load_session(key);
//         option.await
//     }
// }
//
// #[async_trait]
// impl<S, K, Store> SessionStore for Arc<Mutex<Store>>
// where
//     Store: SessionStore<Session = S, Key = K>,
//     S: SessionData,
//     K: SessionKey + Sync,
// {
//     type Session = S;
//     type Key = K;
//
//     async fn create_session(&mut self) -> (Self::Key, Self::Session) {
//         self.lock().await.create_session().await
//     }
//
//     async fn load_session(&self, key: &K) -> Option<S> {
//         self.lock().await.load_session(key).await
//     }
// }

#[async_trait]
pub trait SessionDataValueStorage<K, V>
where
    K: Hash + Eq,
{
    async fn set_value<KVal, VVal>(&self, key: KVal, val: VVal)
    where
        KVal: Into<K> + Send,
        VVal: Into<V> + Send;

    async fn get_value<KRef>(&self, key: &KRef) -> Option<V>
    where
        KRef: Hash + Eq + ?Sized + Sync,
        K: Borrow<KRef> + Hash + Eq;
}

// #[async_trait]
// impl<K, V, T> SessionDataValueStorage<K, V> for Arc<Mutex<T>>
// where
//     K: Hash + Eq,
//     T: SessionDataValueStorage<K, V>,
// {
//     async fn set_value<KVal, VVal>(&mut self, key: KVal, val: VVal)
//     where
//         KVal: Into<K> + Send,
//         VVal: Into<V> + Send,
//     {
//         self.lock().await.set_value(key, val).await
//     }
//
//     async fn get_value<KRef>(&self, key: &KRef) -> Option<V>
//     where
//         KRef: Hash + Eq + ?Sized + Sync,
//         K: Borrow<KRef> + Hash + Eq,
//     {
//         self.lock().await.get_value(key).await
//     }
// }

// pub struct SessionDataMultiStorage<S1, S2> {
//     storage1: S1,
//     storage2: S2,
// }
//
// impl<S1, S2> SessionDataMultiStorage<S1, S2> {
//     pub fn new(storage1: S1, storage2: S2) -> Self {
//         SessionDataMultiStorage { storage1, storage2 }
//     }
// }
//
// impl<K, V, S1, S2> SessionDataValueStorage<K, V> for SessionDataMultiStorage<S1, S2>
// where
//     K: Hash + Eq,
//     S1: SessionDataValueStorage<K, V>,
// {
//     fn set_value<KVal, VVal>(&mut self, key: KVal, val: VVal)
//     where
//         KVal: Into<K>,
//         VVal: Into<V>,
//     {
//         self.storage1.set_value(key, val)
//     }
//
//     fn get_value<KRef>(&self, key: &KRef) -> Option<V>
//     where
//         KRef: Hash + Eq + ?Sized,
//         K: Borrow<KRef> + Hash + Eq,
//     {
//         self.storage1.get_value(key)
//     }
// }
//
// impl<K, V, S1, S2> SessionDataValueStorage<K, V> for SessionDataMultiStorage<S1, S2>
// where
//     K: Hash + Eq,
//     S2: SessionDataValueStorage<K, V>,
// {
//     fn set_value<KVal, VVal>(&mut self, key: KVal, val: VVal)
//     where
//         KVal: Into<K>,
//         VVal: Into<V>,
//     {
//         self.storage2.set_value(key, val)
//     }
//
//     fn get_value<KRef>(&self, key: &KRef) -> Option<V>
//     where
//         KRef: Hash + Eq + ?Sized,
//         K: Borrow<KRef> + Hash + Eq,
//     {
//         self.storage2.get_value(key)
//     }
// }
