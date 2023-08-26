use std::collections::{HashMap, HashSet};

pub enum Error {
    Forbidden,
}

pub struct ProtectedResource<R> {
    resource: R,
}

pub trait Object {
    type Identifier: Eq;

    fn identifier(&self) -> Self::Identifier;
}

pub trait Subject {
    type Role: Eq;

    fn roles(&self) -> HashSet<Self::Role>;
}

pub trait Policy<Res, Subj, Act> {
    fn authorise(&self, resource: &Res, subject: &Subj, action: &Act) -> Result<(), Error>;
}

pub struct RbacPolicy<Res, Subj, Act>
where
    Res: Object,
    Subj: Subject,
{
    allowed: HashMap<(Res::Identifier, Act), HashSet<Subj::Role>>,
}

impl<Res, Subj, Act> RbacPolicy<Res, Subj, Act>
where
    Res: Object,
    Subj: Subject,
{
    pub fn new() -> Self {
        RbacPolicy {
            allowed: HashMap::new(),
        }
    }
}

impl<Res, Subj, Act> Policy<Res, Subj, Act> for RbacPolicy<Res, Subj, Act>
where
    Res: Object,
    <Res as Object>::Identifier: Eq,
    Subj: Subject,
    <Subj as Subject>::Role: Eq,
{
    fn authorise(&self, resource: &Res, subject: &Subj, action: &Act) -> Result<(), Error> {
        let resource_id = resource.identifier();
        let subject_roles = subject.roles();

        // let allowed_roles = self.allowed.get((resource_id, action));
        //
        // if allowed_roles.intersection(subject_roles).length() >= 1 {}

        todo!()
    }
}

// impl<R1, R2, Subject, A1, A2> Policy<Resource, Subject, Action>
//     for RbacPolicy<Resource, Subject, Action>
// {
//     fn authorise(resource: Resource, subject: Subject, action: Action) -> Result<Resource, Error> {
//         todo!()
//     }
// }
