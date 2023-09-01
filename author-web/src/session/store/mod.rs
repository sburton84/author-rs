use crate::session::{SessionData, SessionKey};
use parking_lot::Mutex;
use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;

#[cfg(feature = "in-memory")]
pub mod in_memory;

pub trait SessionStore: Send {
    type Session: SessionData;
    type Key: SessionKey;

    fn create_session(&mut self) -> (Self::Key, Self::Session);
    fn load_session(&self, key: &Self::Key) -> Option<Self::Session>;
}

impl<S, K> SessionStore for Arc<Mutex<dyn SessionStore<Session = S, Key = K>>>
where
    S: SessionData,
    K: SessionKey,
{
    type Session = S;
    type Key = K;

    fn create_session(&mut self) -> (Self::Key, Self::Session) {
        self.lock().create_session()
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

    fn create_session(&mut self) -> (Self::Key, Self::Session) {
        self.lock().create_session()
    }

    fn load_session(&self, key: &K) -> Option<S> {
        self.lock().load_session(key)
    }
}

pub trait SessionDataValueStorage<K, V>
where
    K: Hash + Eq,
{
    fn set_value<KVal, VVal>(&mut self, key: KVal, val: VVal)
    where
        KVal: Into<K>,
        VVal: Into<V>;
    fn get_value<KRef>(&self, key: &KRef) -> Option<V>
    where
        KRef: Hash + Eq + ?Sized,
        K: Borrow<KRef> + Hash + Eq;
}

impl<K, V, T> SessionDataValueStorage<K, V> for Arc<Mutex<T>>
where
    K: Hash + Eq,
    T: SessionDataValueStorage<K, V>,
{
    fn set_value<KVal, VVal>(&mut self, key: KVal, val: VVal)
    where
        KVal: Into<K>,
        VVal: Into<V>,
    {
        self.lock().set_value(key, val)
    }

    fn get_value<KRef>(&self, key: &KRef) -> Option<V>
    where
        KRef: Hash + Eq + ?Sized,
        K: Borrow<KRef> + Hash + Eq,
    {
        self.lock().get_value(key)
    }
}

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
