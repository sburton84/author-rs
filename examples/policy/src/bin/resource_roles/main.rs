use assert_matches::assert_matches;
use author::rbac::{GlobalRbacPolicy, GlobalRbacSubject, RbacResource, RbacResourceWithRole};
use author::{Policy, Resource, Subject};
use std::collections::HashSet;

struct User {
    name: String,
    roles: HashSet<GlobalRole>,
}

impl Subject for User {}

impl GlobalRbacSubject for User {
    type GlobalRole = GlobalRole;

    fn global_roles(&self) -> HashSet<Self::GlobalRole> {
        self.roles.clone()
    }
}

struct Customer {
    name: String,
}

impl Resource for Customer {
    type Action = CustomerAction;
}

impl RbacResourceWithRole for Customer {
    type Role = CustomerRole;
}

impl RbacResource<CustomerRole> for Customer {
    fn allowed_roles(&self, action: &Self::Action) -> HashSet<CustomerRole> {
        match action {
            CustomerAction::Read => HashSet::from([
                CustomerRole::Admin,
                CustomerRole::Other,
                CustomerRole::Owner,
            ]),
            CustomerAction::Write => HashSet::from([CustomerRole::Admin, CustomerRole::Owner]),
        }
    }
}

impl RbacResource<GlobalRole> for Customer {
    fn allowed_roles(&self, action: &Self::Action) -> HashSet<GlobalRole> {
        match action {
            CustomerAction::Read => HashSet::from([GlobalRole::Admin]),
            CustomerAction::Write => HashSet::from([GlobalRole::Admin]),
        }
    }
}

struct Product {
    name: String,
    cost: f32,
}

impl Resource for Product {
    type Action = ProductAction;
}

impl RbacResource<GlobalRole> for Product {
    fn allowed_roles(&self, action: &Self::Action) -> HashSet<GlobalRole> {
        match action {
            ProductAction::Read => HashSet::from([GlobalRole::Admin, GlobalRole::User]),
            ProductAction::Write => HashSet::from([GlobalRole::Admin]),
            ProductAction::Delete => HashSet::from([GlobalRole::Admin]),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum CustomerAction {
    Read,
    Write,
}

#[derive(PartialEq, Eq, Hash)]
enum ProductAction {
    Read,
    Write,
    Delete,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum GlobalRole {
    User,
    Admin,
}

#[derive(PartialEq, Eq, Hash)]
enum CustomerRole {
    Admin,
    Owner,
    Other,
}

fn main() -> anyhow::Result<()> {
    let policy = GlobalRbacPolicy::new();

    let user = User {
        name: "User".to_string(),
        roles: HashSet::from([GlobalRole::User]),
    };

    let admin_user = User {
        name: "Admin".to_string(),
        roles: HashSet::from([GlobalRole::User, GlobalRole::Admin]),
    };

    let customer = Customer {
        name: "Customer".to_string(),
    };

    let product = Product {
        name: "Product".to_string(),
        cost: 0.50,
    };

    assert_matches!(
        policy.authorise(&customer, &user, &CustomerAction::Read),
        Err(_)
    );

    assert_matches!(
        policy.authorise(&customer, &user, &CustomerAction::Write),
        Err(_)
    );

    assert_matches!(
        policy.authorise(&customer, &admin_user, &CustomerAction::Read),
        Ok(_)
    );

    assert_matches!(
        policy.authorise(&customer, &admin_user, &CustomerAction::Write),
        Ok(_)
    );

    Ok(())
}
