mod user;
mod login_action;
use std::collections::HashMap;
pub use user::User;
pub use login_action::*;

pub mod serde {
    pub use serde::*;
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn build_users_file() {
    use std::io::Write;

    let users = get_users_old();
    //let json = serde_json::to_string(&users).unwrap();
    let json = serde_json::to_string_pretty(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

pub fn save_users_file(users: &HashMap<String, User>) {
    use std::io::Write;
    let json = serde_json::to_string_pretty(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

#[allow(dead_code)]
fn get_users_old() -> HashMap<String, User> {
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

pub fn get_users() -> HashMap<String, User> {
    let json = std::fs::read_to_string("users.json").unwrap();
    serde_json::from_str(&json).unwrap()
}

pub fn login(users: &HashMap<String, User>, username: &str, password: &str) -> Option<LoginAction> {
    let username = username.trim().to_lowercase();
    let password = hash_password(password.trim());

    users
        .get(&username)
        .filter(|user| user.password == password)
        .map(|user| user.action.clone())
}



