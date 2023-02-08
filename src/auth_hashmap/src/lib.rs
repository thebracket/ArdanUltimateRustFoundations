use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub action: LoginAction,
}

impl User {
    pub fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            action
        }
    }
}

pub fn get_users() -> HashMap<String, User> {
    /*let mut result = HashMap::new();
    result.insert("herbert".to_string(), User::new("herbert", "password", LoginAction::Accept(Role::Admin)));
    result*/
    let mut users = vec![
        User::new("herbert", "password", LoginAction::Accept(Role::Admin)),
        User::new("bob", "password", LoginAction::Accept(Role::User)),
        User::new("fred", "password", LoginAction::Denied(DeniedReason::PasswordExpired)),
    ];
    /*users
        .iter() // Create an iterator
        .map(|user| (user.username.clone(), user.clone()) )
        .collect()*/
    users
        .drain(0..)
        .map(|user| ( user.username.clone(), user ))
        .collect()
}

pub fn login(users: &HashMap<String, User>, username: &str, password: &str) -> Option<LoginAction> {
    let username = username.trim().to_lowercase();
    let password = password.trim();

    users
        .get(&username)
        .filter(|user| user.password == password)
        .map(|user| user.action.clone())
}

#[derive(PartialEq, Debug, Clone)]
pub enum Role {
    Admin,
    User,
    Limited
}

#[derive(PartialEq, Debug, Clone)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked{reason: String},
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}

impl LoginAction {
    pub fn do_login(&self, on_success: fn(&Role), on_denied: fn(&DeniedReason)) {
        match self {
            Self::Accept(role) => on_success(role),
            Self::Denied(reason) => on_denied(reason),
        }
    }
}
