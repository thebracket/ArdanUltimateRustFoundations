pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "herbert"
}

#[derive(PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
    Limited
}

#[derive(PartialEq, Debug)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked{reason: String},
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}

pub fn login(name: &str) -> Option<LoginAction> {
    match name.to_lowercase().trim() {
        "herbert" => Some(LoginAction::Accept(Role::Admin)),
        "bob" => Some(LoginAction::Accept(Role::User)),
        "fred" => Some(LoginAction::Denied(DeniedReason::PasswordExpired)),
        "kevin" => Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: "Call Human Resources!".to_string() })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Herbert", greet_user("Herbert"));
    }

    #[test]
    fn test_case_and_trim() {
        assert!(is_login_allowed("HeRbErT"));
        assert!(is_login_allowed("  herbert\r\n"));
    }

    #[test]
    fn test_login_fail() {
        assert!(!is_login_allowed("bob"));
    }

    #[test]
    fn test_enums() {
        assert_eq!(login("Herbert"), Some(LoginAction::Accept(Role::Admin)));
        assert_eq!(login("bob"), Some(LoginAction::Accept(Role::User)));
        assert_eq!(login("fred"), Some(LoginAction::Denied(DeniedReason::PasswordExpired)));
        assert_eq!(login("anonymous"), None);
        if let Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: _ })) = login("kevin") {
            // All is well
        } else {
            panic!("Failed to read kevin");
        }
    }
}
