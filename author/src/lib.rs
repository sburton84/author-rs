pub enum Error {
    Forbidden,
}

pub struct ProtectedResource<R> {
    resource: R,
}

pub trait Object<Identifier> {
    fn identifier() -> Identifier;
}

pub trait Subject<Role> {
    fn roles<'a>() -> &'a [Role];
}

pub trait Policy<Resource, Subject, Action> {
    fn authorise(resource: &Resource, subject: &Subject, action: &Action) -> Result<(), Error>;
}

// pub struct RbacPolicy<Resource, Subject, Action, ResourceIdentifier, Role>
// where
//     Subject: Subject<Role>,
// {
//     allowed: Vec<(ResourceIdentifier, Vec<Role>, Action)>,
// }
//
// impl<Resource, Subject, Action, ResourceIdentifier>
//     RbacPolicy<Resource, Subject, Action, ResourceIdentifier>
// {
//     pub fn new() -> Self {
//         RbacPolicy {
//             allowed: Vec::new(),
//         }
//     }
// }
//
// impl<Resource, Subject, Action, ResourceIdentifier> Policy<Resource, Subject, Action>
//     for RbacPolicy<Resource, Subject, Action, ResourceIdentifier>
// where
//     Resource: Object<ResourceIdentifier>,
// {
//     fn authorise(resource: &Resource, subject: &Subject, action: &Action) -> Result<(), Error> {
//         todo!()
//     }
// }

// impl<R1, R2, Subject, A1, A2> Policy<Resource, Subject, Action>
//     for RbacPolicy<Resource, Subject, Action>
// {
//     fn authorise(resource: Resource, subject: Subject, action: Action) -> Result<Resource, Error> {
//         todo!()
//     }
// }
