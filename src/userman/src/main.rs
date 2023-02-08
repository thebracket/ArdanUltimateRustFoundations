use auth_userman::*;
use clap::{Parser, Subcommand};
use std::collections::HashMap;

type UserMap = HashMap<String, User>;

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user.
    Add {
        /// Username
        #[arg(long)]
        username: String,

        /// Password
        #[arg(long)]
        password: String,

        /// Optional - mark as a limited user
        #[arg(long)]
        limited: Option<bool>,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// Username
        username: String,
    },
    /// Change a password
    ChangePassword {
        /// Username
        username: String,

        /// New Password
        new_password: String,
    }
}

fn list_users(users: &UserMap) {
    use colored::Colorize;
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    users.iter().for_each(|(_, user)| {
        let action = format!("{:?}", user.action);
        let action = match user.action {
            LoginAction::Accept(..) => action.green(),
            LoginAction::Denied(..) => action.red(),
        };
        println!("{:<20}{:<20}", user.username, action);
    });
}

fn add_user(
    users: &mut UserMap,
    username: String,
    password: String,
    limited: Option<bool>,
    admin: Option<bool>,
) {
    if users.contains_key(&username) {
        println!("{username} already exists, aborting.");
        return;
    }
    let action = LoginAction::Accept(if limited.is_some() {
        Role::Limited
    } else if admin.is_some() {
        Role::Admin
    } else {
        Role::User
    });
    let user = User::new(&username, &password, action);
    users.insert(username, user);
    save_users_file(users);
}

fn delete_user(users: &mut UserMap, username: String) {
    if !users.contains_key(&username) {
        println!("{username} does not exist, aborting");
        return;
    }
    users.remove(&username);
    save_users_file(users);
}

fn change_password(users: &mut UserMap, username: String, new_password: String) {
    if let Some(mut user) = users.get_mut(&username) {
        user.password = hash_password(&new_password);
        save_users_file(users);
    } else {
        println!("{username} does not exist, aborting");
    }
}

fn main() {
    let mut users = get_users();
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users(&users);
        }
        Some(Commands::Add {
            username,
            password,
            limited,
            admin,
        }) => {
            add_user(&mut users, username, password, limited, admin);
        }
        Some(Commands::ChangePassword { username, new_password }) => {
            change_password(&mut users, username, new_password);
        }
        Some(Commands::Delete { username }) => {
            delete_user(&mut users, username);
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }
}
