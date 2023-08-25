use std::str::FromStr;

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
