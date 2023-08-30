use crate::{Error, Object, Policy, Resource, Subject};
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::Extend;

pub mod config;

pub trait GlobalRbacSubject: Subject {
    type GlobalRole: Hash + Eq;

    fn global_roles(&self) -> HashSet<Self::GlobalRole> {
        HashSet::new()
    }
}

pub trait RbacSubject: Subject {
    fn resource_roles<Res>(&self, resource: &Res) -> HashSet<Res::Role>
    where
        Res: RbacResourceWithRole,
    {
        HashSet::new()
    }
}

pub trait RbacResourceWithRole: RbacResource<Self::Role> {
    type Role: Hash + Eq;
}

pub trait RbacResource<Role>: Resource {
    fn allowed_roles(&self, action: &Self::Action) -> HashSet<Role> {
        HashSet::new()
    }
}

pub struct GlobalRbacPolicy {}

impl GlobalRbacPolicy {
    pub fn new() -> Self {
        GlobalRbacPolicy {}
    }
}

impl<Res, Subj> Policy<Res, Subj> for GlobalRbacPolicy
where
    Subj: GlobalRbacSubject,
    Res: RbacResource<Subj::GlobalRole>,
{
    fn authorise(&self, resource: &Res, subject: &Subj, action: &Res::Action) -> Result<(), Error> {
        let subject_global_roles = subject.global_roles();
        let allowed_global_roles = RbacResource::allowed_roles(resource, action);

        let matching_roles: HashSet<_> = allowed_global_roles
            .intersection(&subject_global_roles)
            .collect();

        if matching_roles.len() == 0 {
            return Err(Error::Forbidden);
        }

        Ok(())
    }
}

// pub struct ResourceRbacPolicy<Res, Role, Act>
// where
//     Res: Resource + Object,
// {
//     pub(crate) config: RbacResourcePolicyConfig<Res::Identifier, Role, Act>,
// }
//
// pub struct RbacPolicyConfig<ResourceIdentifier, Role, Act> {
//     pub(crate) global_allowed: GlobalRbacPolicyConfig<Role, Act>,
//     pub(crate) allowed: HashMap<TypeId, RbacResourcePolicyConfig<ResourceIdentifier, Role, Act>>,
// }

// pub struct RbacPolicy<Res, Subj, Act>
// where
//     Res: Object,
//     Subj: Subject,
// {
//     config: RbacPolicyConfig<Res::Identifier, Subj::Role, Act>,
// }
//
// impl<Res, Subj, Act> RbacPolicy<Res, Subj, Act>
// where
//     Res: Object,
//     Subj: Subject,
// {
//     pub fn new() -> Self {
//         RbacPolicy {
//             config: RbacPolicyConfig::new(),
//         }
//     }
// }
//
// impl<Res, Subj, Act> Policy for RbacPolicy<Res, Subj, Act>
// where
//     Res: Object,
//     <Res as Object>::Identifier: Hash + Eq,
//     Subj: Subject,
//     <Subj as Subject>::Role: Hash + Eq,
//     Act: Hash + Eq,
// {
//     fn authorise<Res, Subj>(
//         &self,
//         resource: &Res,
//         subject: &Subj,
//         action: &Res::Action,
//     ) -> Result<(), Error> {
//         let resource_id = resource.identifier();
//         let subject_roles = subject.roles();
//
//         let allowed_roles = self
//             .config
//             .allowed
//             .get(&resource_id)
//             .and_then(|a| a.get(&action));
//
//         let allowed_roles = match allowed_roles {
//             Some(a) => a,
//             None => {
//                 return Err(Error::Forbidden);
//             }
//         };
//
//         let matching_roles: HashSet<_> = allowed_roles.intersection(&subject_roles).collect();
//
//         if matching_roles.len() == 0 {
//             return Err(Error::Forbidden);
//         }
//
//         Ok(())
//     }
// }

// impl<R1, R2, Subject, A1, A2> Policy<Resource, Subject, Action>
//     for RbacPolicy<Resource, Subject, Action>
// {
//     fn authorise(resource: Resource, subject: Subject, action: Action) -> Result<Resource, Error> {
//         todo!()
//     }
// }
