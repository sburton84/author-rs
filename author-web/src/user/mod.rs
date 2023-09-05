use crate::session::store::in_memory::InMemorySessionData;
use crate::session::store::SessionDataValueStorage;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UserSession {
    type User;

    async fn set_user(&mut self, user: Self::User) -> anyhow::Result<()>;
    async fn current_user(&self) -> anyhow::Result<Option<Self::User>>;
}

#[cfg(feature = "in-memory")]
#[async_trait]
impl<U> UserSession for InMemorySessionData<String, U>
where
    U: Clone + Send,
{
    type User = U;

    async fn set_user(&mut self, user: U) -> anyhow::Result<()> {
        Ok(self.set_value("current_user", user).await?)
    }

    async fn current_user(&self) -> anyhow::Result<Option<Self::User>> {
        Ok(self.get_value("current_user").await?)
    }
}

#[async_trait]
impl<U, Sess> UserSession for Arc<Sess>
where
    Sess: UserSession<User = U> + Send + Sync,
    U: Clone + Send + 'static,
{
    type User = U;
    async fn set_user(&mut self, user: U) -> anyhow::Result<()> {
        Ok(self.set_user(user).await?)
    }

    async fn current_user(&self) -> anyhow::Result<Option<Self::User>> {
        Ok(self.current_user().await?)
    }
}
