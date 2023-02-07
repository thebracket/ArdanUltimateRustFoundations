use auth_if::is_login_allowed;

fn main() {
    println!("Welcome to the (Not Very) Secure Server");
    println!("Enter your username:");
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();
    if is_login_allowed(&input) {
        println!("Welcome, {input}");
    } else {
        println!("Sorry, {input} is now allowed");
    }
}
