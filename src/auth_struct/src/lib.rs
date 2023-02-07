pub struct User {
    username: String,
    password: String,
    action: LoginAction,
}

impl User {
    fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            action
        }
    }
}

pub fn get_users() -> [User; 3] {
    [
        User::new("herbert", "password", LoginAction::Accept(Role::Admin)),
        User::new("bob", "password", LoginAction::Accept(Role::User)),
        User::new("fred", "password", LoginAction::Denied(DeniedReason::PasswordExpired)),
    ]
}

pub fn login(users: &[User], username: &str, password: &str) -> Option<LoginAction> {
    let username = username.trim().to_lowercase();
    let password = password.trim();
    users
        .iter()
        .find(|u| u.username == username && u.password == password).map(|user| user.action.clone())
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
    fn standard_user() -> Option<Self> {
        Some(LoginAction::Accept(Role::User))
    }

    pub fn do_login(&self, on_success: fn(&Role), on_denied: fn(&DeniedReason)) {
        match self {
            Self::Accept(role) => on_success(role),
            Self::Denied(reason) => on_denied(reason),
        }
    }
}
