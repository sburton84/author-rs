use std::any::TypeId;
use std::collections::{HashMap, HashSet};

// pub struct RbacPolicyConfig<ResourceIdentifier, Role, Act> {
//     pub(crate) global_allowed: GlobalRbacPolicyConfig<Role, Act>,
//     pub(crate) allowed: HashMap<TypeId, RbacResourcePolicyConfig<ResourceIdentifier, Role, Act>>,
// }

// impl<ResourceIdentifier, Role, Act> RbacPolicyConfig<ResourceIdentifier, Role, Act> {
//     pub fn new() -> Self {
//         RbacPolicyConfig {
//             allowed: HashMap::new(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn load_from_yaml() {
        let config_yaml = r#"
            resources:
              - customer:
                  actions:
                    - read:
                      allowed roles: [admin]
              - user:
        "#;
    }
}
