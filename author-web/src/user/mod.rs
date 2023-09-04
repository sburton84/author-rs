use crate::session::store::in_memory::InMemorySessionData;
use crate::session::store::SessionDataValueStorage;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UserSession<U> {
    async fn set_user(&self, user: U);
    async fn current_user(&self) -> Option<U>;
}

#[cfg(feature = "in-memory")]
#[async_trait]
impl<U> UserSession<U> for InMemorySessionData<String, U>
where
    U: Clone + Send,
{
    async fn set_user(&self, user: U) {
        self.set_value("current_user", user).await
    }

    async fn current_user(&self) -> Option<U> {
        self.get_value("current_user").await
    }
}

#[async_trait]
impl<U, Sess> UserSession<U> for Arc<Sess>
where
    Sess: UserSession<U> + Send + Sync,
    U: Clone + Send + 'static,
{
    async fn set_user(&self, user: U) {
        self.set_user(user).await
    }

    async fn current_user(&self) -> Option<U> {
        self.current_user().await
    }
}
