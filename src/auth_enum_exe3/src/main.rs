use auth_enum3::{login, LoginAction, Role, DeniedReason};

fn main() {
    println!("Welcome to the (Not Very) Secure Server");
    println!("Enter your username:");
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    match login(&input) {
        None => {
            println!("{} is not a known user.", input.trim());
            println!("This is where we handle new users.");
        }
        Some(LoginAction::Accept(role)) => {
            println!("Welcome: {}", input.trim());
            match role {
                Role::Admin => println!("With great power, comes great responsibility."),
                Role::Limited => println!("You have read-only access."),
                Role::User => println!("You have regular user access"),
            }
        }
        Some(LoginAction::Denied(DeniedReason::PasswordExpired)) => {
            println!("Unfortunately, your password has expired.");
            println!("It needs to 800 characters long and contain an entire sonnet.");
        }
        Some(LoginAction::Denied(DeniedReason::AccountLocked { reason })) => {
            println!("Sorry, {}, your login was denied.", input.trim());
            println!("Reason: {reason}");
        }
    }
}
