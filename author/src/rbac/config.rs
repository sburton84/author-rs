use std::any::TypeId;
use std::collections::{HashMap, HashSet};

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
