use std::collections::HashSet;
use std::hash::Hash;
use thiserror::Error;

pub mod rbac;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Forbidden")]
    Forbidden,
}

pub struct ProtectedResource<R> {
    resource: R,
}

pub trait Object {
    type Identifier: Hash + Eq;

    fn identifier(&self) -> Self::Identifier;
}

pub trait Resource {
    type Action: Hash + Eq;

    //fn authorise<Subj>(&self, subject: &Subj, action: &Self::Action) -> Result<(), Error>;
}

pub trait Subject {}

pub trait Policy<Res, Subj>
where
    Res: Resource,
    Subj: Subject,
{
    fn authorise(&self, resource: &Res, subject: &Subj, action: &Res::Action) -> Result<(), Error> {
        Err(Error::Forbidden)
    }
}
