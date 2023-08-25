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

    fn roles<'a>(&'a self) -> &'a [Self::Role];
}

pub trait Policy<Res, Subj, Act> {
    fn authorise(resource: &Res, subject: &Subj, action: &Act) -> Result<(), Error>;
}

pub struct RbacPolicy<Res, Subj, Act>
where
    Res: Object,
    Subj: Subject,
{
    allowed: Vec<(Res::Identifier, Vec<Subj::Role>, Act)>,
}

impl<Res, Subj, Act> RbacPolicy<Res, Subj, Act>
where
    Res: Object,
    Subj: Subject,
{
    pub fn new() -> Self {
        RbacPolicy {
            allowed: Vec::new(),
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
    fn authorise(resource: &Res, subject: &Subj, action: &Act) -> Result<(), Error> {
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
