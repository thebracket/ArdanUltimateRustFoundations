# Build a Command-Line User Management App

> This is live-coded. The Github version is [here]()

## Create a new project

1. Go to your workspace root, in my case `c:\users\herbert\rust\live`.
2. Create a new project, `cargo new userman`.
3. Edit `Cargo.toml` to include `userman` in your workspace `[members]`.

## Setup Dependencies

Open your newly created `userman/Cargo.toml` file. You need to depend upon the `authentication` library you've been working on. We're also going to make use of a really handy Rust crate called `clap`, which is rapidly becoming the standard way of reading command-line options. Once again, we want to make use of some "derive" macros, so we'll include that feature.:

```toml
[package]
name = "userman"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
authentication = { path = "../authentication" }
clap = { version = "4", features = ["derive"] }
```

The "derive" features make supporting command-line options quite straightforward.

## A Minimal Clap Example

Let's replace "Hello World" with a starting point for a `clap` driven application:

```rust
use clap::{Parser, Subcommand};

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
}


fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            println!("All Users Goes Here\n");
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }    
}

```

Working through this program:

* `#[derive(Parser)]` adds Clap parser code to a structure.
* `#[command()]` indicates that this is the default command.
* `#[command(subcommand)]` tells Clap that we're defining additional command-line options, and they are listed in the `Commands` enum.
* `#[derive(Subcommand)]` tells Clap that an enumeration contains sub-commands.
* Each enumeration entry represents one possible command.
* Procedural macros can read your comments! `///` is a *documentation comment*---used if you use `cargo doc` to document your program.
* `let cli = Args::parse()` tells Clap to start, by reading the incoming command structure.
* Finally, we match the returned `cli` object to see if a command was issued.

Let's give it a go.

```
cargo run

Compiling userman v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\userman)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\userman.exe`
Run with --help to see instructions
```

Passing arguments to `cargo run` requires a slightly strange syntax. Let's pass `--help`, as instructed:

```
cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\userman.exe --help`
Usage: userman.exe [COMMAND]

Commands:
  list  List all users
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

And let's verify that the `--list` command prints out our message:

```
cargo run -- list  
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\userman.exe list`
All Users Goes Here
```

Clap has parsed your command enum, read your comments, and built a basic command-line system for you.

## Displaying a List of Users

Start by copying `users.json` from your `login` directory into `userman`. That saves us from having to mess with paths.

Now we'll load the users list at the beginning of the program:

```rust
use auth_passwords::get_users;

let mut users = get_users();
```

We know we're going to change `users` eventually, so we'll make it mutable and ignore the warnings for now.

I don't know about you, but I'm sick of typing `HashMap<String, User>` over and over. Let's make a *type alias*:

```rust
type UserMap = HashMap<String, User>;
```

Type aliases are a handy way to reduce typing. They don't *create* a new type---they just make them easier to type.

So now let's start to build a user list function:

```rust
fn list_users(users: &UserMap) {
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    users
        .iter()
        .for_each(|(_, user)| {
            println!("{:<20}{:<20?}", user.username, user.action);
        });
}
```

There's some new stuff here:
* `{:<20}` in a format string means "left align this field and pad to be 20 characters wide.
* `{:-<40}` in a format string means "use `-` to pad left 40 characters wide. It's a handy way to draw a line of minuses.
* We're calling `iter()` on a `HashMap`. This returns the full *tuple* of each entry: `(key, value)`.
* When we call `for_each`, we don't actually care about the key - so we mark it as a placeholder with `_`.
* Then we print each user.
* `{:<20?}` is a combination of format functions we've used before. `:` means "this is a command". `<20` means left-align and pad to 20 characters wide. `?` means "display debug formatting".

The output now looks like this:

```
Username            Login Action        
----------------------------------------
bob                 Accept(User)
fred                Denied(PasswordExpired)
herbert             Accept(Admin)
```

That's not bad for 8 lines of code!

## Improving the List of Users

Let's add a little color. Denied users show up as red. Open `Cargo.toml` and add the following dependency:

```toml
[dependencies]
auth_passwords = { path = "../auth_passwords" }
clap = { version = "4", features = ["derive"] }
colored = "2.0.0"
```

And let's modify the function a little:

```rust
fn list_users(users: &UserMap) {
    use colored::Colorize;
    println!("{:<20}{:<20}", "Username", "Login Action");
    println!("{:-<40}", "");

    users
        .iter()
        .for_each(|(_, user)| {
            let action = format!("{:?}", user.action);
            let action = match user.action {
                LoginAction::Accept(..) => action.green(),
                LoginAction::Denied(..) => action.red(),
            };
            println!("{:<20}{:<20}", user.username, action);
        });
}
```

Now when you list users, they are color coded:

![](/images/ColoredUsers.png)

## Adding Users

> Live-Coding. The GitHub version of the auth library is [here](/src/auth_userman/)

Let's pop into our `authentication` project and add one more function:

```rust
pub fn save_users_file(users: &HashMap<String, User>) {
    use std::io::Write;
    let json = serde_json::to_string_pretty(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}
```

This is a copy/paste of `build_users`, but accepts a user list to save instead of making a new one.

Now, we go back to `userman`.

Let's build a subcommand for adding a user:

```rust
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
  }
}
```

Now we need to handle it:

```rust
fn main() {
    let mut users = get_users();
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users(&users);
        }
        Some(Commands::Add { username, password, limited, admin }) => {
            add_user(&mut users, username, password, limited, admin);
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }    
}  
```

And finally, let's write the `add_user` function:

```rust
fn add_user(users: &mut UserMap, username: String, password: String, limited: Option<bool>, admin: Option<bool>) {
    let action = LoginAction::Accept(
        if limited.is_some() {
            Role::Limited
        } else if admin.is_some() {
            Role::Admin
        } else {
            Role::User
        }
    );
    let user = User::new(&username, &password, action);
    users.insert(username, user);
    save_users_file(users);
}
```

Notice that we're making use of Rust's ability to include expressions almost anywhere. The parameter to `Accept` is itself an `if` statement. Then we insert a user into the user list, and save the result.

Let's give it a try:

```
cargo run -- --help

Usage: userman.exe [COMMAND]

Commands:
  list  List all users
  add   Add a user
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

The new command has appeared. Does it support help?

```
cargo run -- add --help

Add a user

Usage: userman.exe add [OPTIONS] --username <USERNAME> --password <PASSWORD>

Options:
      --username <USERNAME>  Username
      --password <PASSWORD>  Password
      --limited <LIMITED>    Optional - mark as a limited user [possible values: true, false]
      --admin <ADMIN>        Optional - mark as an admin [possible values: true, false]
  -h, --help                 Print help
```

Yes, it does! All of the options you've created are included. Let's try adding a user.

```
cargo run -- add --username chester --password tester --limited true

cargo run -- list

Username            Login Action
----------------------------------------
herbert             Accept(Admin)
chester             Accept(Limited)
bob                 Accept(User)
fred                Denied(PasswordExpired)
```

Chester has appeared! Adding users works. We can check the `users.json` file, too:

```json
  "chester": {
    "username": "chester",
    "password": "9BBA5C53A0545E0C80184B946153C9F58387E3BD1D4EE35740F29AC2E718B019",
    "action": {
      "Accept": "Limited"
    }
  },
```

### Duplicate Users

Let's add one more thing to `add_user` - we'll check to see if the user already exists, and refuse to add them if they do. Otherwise, we're just replacing users.

```rust
if users.contains_key(&username) {
    println!("{username} already exists, aborting.");
    return;
}
```

## Deleting Users

Another useful command is deleting users. We'll follow a very similar procedure. First, we add the command:

```rust
/// Delete a user
Delete {
    /// Username
    username: String,
}
```

Note that this time we haven't added an `#[arg]`---we're just going to take a string directly. I'm using different approaches to illustrate how to use `clap`.

Let's adjust the `cli` match statement to handle user deletion:

```rust
Some(Commands::Delete { username }) => {
    delete_user(&mut users, username);
}
```

And finally, we'll write the function:

```rust
fn delete_user(users: &mut UserMap, username: String) {
    if !users.contains_key(&username) {
        println!("{username} does not exist, aborting");
        return;
    }
    users.remove(&username);
    save_users_file(users);
}
```

Let's test this by removing `chester`:

```
cargo run -- delete chester
cargo run -- list

Username            Login Action
----------------------------------------
fred                Denied(PasswordExpired)
bob                 Accept(User)
herbert             Accept(Admin)
```

Chester has been consigned to the bit-bucket (removed). You may want to add a confirmation!

## Updating Users

We've got the `C`, `R` and `D` done for `CRUD` (Create-Read-Update_Delete). Let's add a few of options for changing users.

### Changing passwords

The command:

```rust
/// Change a password
ChangePassword {
    /// Username
    username: String,

    /// New Password
    new_password: String,
}
```

The `cli` match:

```rust
Some(Commands::ChangePassword { username, new_password }) => {
    change_password(&mut users, username, new_password);
}
```

And the action function:

```rust
fn change_password(users: &mut UserMap, username: String, new_password: String) {
    if let Some(mut user) = users.get_mut(&username) {
        user.password = hash_password(&new_password);
        save_users_file(users);
    } else {
        println!("{username} does not exist, aborting");
    }
}
```

Notice how we're using a new function - `get_mut` to return a *mutable* reference to a record in the `HashMap`. Using `if let` to find it, and `else` to abort if there's no match lets us condense the function quite nicely.

Other functions are a variation on this theme.