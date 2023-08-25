use crate::session::{SessionData, SessionKey};
use parking_lot::Mutex;
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
