use crate::session::store::in_memory::InMemorySessionData;
use crate::session::store::SessionDataValueStorage;
use parking_lot::Mutex;
use std::sync::Arc;

pub trait UserSession<U> {
    fn set_user(&mut self, user: &U);
    fn current_user(&self) -> Option<U>;
}

#[cfg(feature = "in-memory")]
impl<U> UserSession<U> for InMemorySessionData<String, U>
where
    U: Clone,
{
    fn set_user(&mut self, user: &U) {
        self.set_value("current_user", user.clone())
    }

    fn current_user(&self) -> Option<U> {
        self.get_value("current_user")
    }
}

impl<U, Sess> UserSession<U> for Arc<Mutex<Sess>>
where
    Sess: UserSession<U>,
{
    fn set_user(&mut self, user: &U) {
        self.lock().set_user(user)
    }

    fn current_user(&self) -> Option<U> {
        self.lock().current_user()
    }
}
