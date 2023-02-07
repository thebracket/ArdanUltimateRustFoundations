pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "herbert"
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Admin,
    User,
    Denied,
}

pub fn login(name: &str) -> LoginAction {
    match name.to_lowercase().trim() {
        "herbert" => LoginAction::Admin,
        "bob" | "fred" => LoginAction::User,
        _ => LoginAction::Denied,
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
        assert_eq!(login("Herbert"), LoginAction::Admin);
        assert_eq!(login("bob"), LoginAction::User);
        assert_eq!(login("fred"), LoginAction::User);
        assert_eq!(login("anonymous"), LoginAction::Denied);
    }
}
