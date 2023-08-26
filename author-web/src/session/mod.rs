use parking_lot::Mutex;
use std::borrow::Borrow;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::Arc;

pub mod store;

pub trait SessionKey: FromStr {
    fn generate() -> Self;
}

pub trait SessionData: Send + Sync {
    fn new() -> Self;
}

pub trait SessionSubject<Subject> {
    fn subject() -> Subject;
}

impl<S> SessionData for Arc<Mutex<S>>
where
    S: SessionData,
{
    fn new() -> Self {
        Arc::new(Mutex::new(S::new()))
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
