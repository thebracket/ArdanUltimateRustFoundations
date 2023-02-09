use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked{reason: String},
}