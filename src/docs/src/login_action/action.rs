use serde::{Serialize, Deserialize};
use crate::{Role, DeniedReason};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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
