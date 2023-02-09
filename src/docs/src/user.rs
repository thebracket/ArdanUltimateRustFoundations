use serde::{Serialize, Deserialize};
use crate::{LoginAction, hash_password};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub(crate) password: String,
    pub action: LoginAction,
}

impl User {
    pub fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(),
            password: hash_password(password),
            action
        }
    }
}