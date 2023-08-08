use uuid::Uuid;

pub struct Session {
    uuid: Uuid,
}

pub trait SessionStore: Send + Sync {
    fn create_session(&self) -> Session;
    fn load_session(&self, uuid: Uuid) -> Session;
}

pub struct SessionConfig {
    cookie_name: String,
}
