use uuid::Uuid;

pub struct Session {
    uuid: Uuid,
}

pub trait SessionStore {
    fn create_session() -> Session;
    fn load_session(uuid: Uuid) -> Session;
}

pub struct SessionConfig {
    cookie_name: String,
}
