pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "herbert"
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
}
