pub enum Error {
    Forbidden,
}

// pub trait Policy<Resource, Subject, Action> {
//     fn authorise(resource: Resource, subject: Subject, action: Action) -> Result<Resource, Error> {}
// }
//
// pub struct RbacPolicy<Resource, Subject, Action> {}
//
// impl<Resource, Subject, Action> RbacPolicy<Resource, Subject, Action> {
//     pub fn new() -> Self {
//         RbacPolicy {}
//     }
// }
//
// impl<Resource, Subject, Action> Policy<Resource, Subject, Action>
//     for RbacPolicy<Resource, Subject, Action>
// {
//     fn authorise(resource: Resource, subject: Subject, action: Action) -> Result<Resource, Error> {}
// }
//
// // impl<R1, R2, Subject, A1, A2> Policy<Resource, Subject, Action>
// // for RbacPolicy<Resource, Subject, Action>
// // {
// //     fn authorise(resource: Resource, subject: Subject, action: Action) -> Result<Resource, Error> {}
// // }
//
// pub trait Subject<R> {
//     fn get_roles<'a>() -> &'a [R];
// }
