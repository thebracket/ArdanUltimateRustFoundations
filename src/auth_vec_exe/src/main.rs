use auth_vec::*;

fn user_accepted(role: &Role) {
    println!("You are logged in as a {role:?}");
}

fn main() {
    let mut users = get_users();
    users.push(User::new("kent", "password", LoginAction::Accept(Role::Limited)));
    users.remove(0);
    users.retain(|u| u.username == "kent");

    let usernames: Vec<&String> = users
        .iter()
        .map(|u| &u.username)
        .collect();
    println!("{usernames:#?}");

    println!("Welcome to the (Not Very) Secure Server");
    println!("Enter your username:");
    let mut username = String::new();
    let mut password = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut username).unwrap();
    println!("Enter your password:");
    stdin.read_line(&mut password).unwrap();

    match login(&users, &username, &password) {
        None => {
            println!("{} is not a known user.", username.trim());
            println!("This is where we handle new users.");
        }
        Some(login_action) => {
            login_action.do_login(
                user_accepted, 
                |reason| {
                    println!("Access denied");
                    println!("{reason:?}");
                }
            )
        }
    }
}
