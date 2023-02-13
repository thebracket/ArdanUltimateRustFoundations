# Structures

> This session is live coded. The GitHub repo entry is [here](/src/auth_struct/)

Storing your company's login directory in a collection of `if` statements works while you are still a company of 3 people, but becomes painful when you grow.

Let's start by trimming out some of the leftovers in our `authentication` project. This reduces our starting point to a more manageable:

```rust
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

impl LoginAction {
    fn standard_user() -> Option<Self> {
        Some(LoginAction::Accept(Role::User))
    }

    pub fn do_login(&self, on_success: fn(&Role), on_denied: fn(&DeniedReason)) {
        match self {
            Self::Accept(role) => on_success(role),
            Self::Denied(reason) => on_denied(reason),
        }
    }
}

pub fn login(name: &str) -> Option<LoginAction> {
    match name.to_lowercase().trim() {
        "herbert" => Some(LoginAction::Accept(Role::Admin)),
        "bob" => LoginAction::standard_user(),
        "fred" => Some(LoginAction::Denied(DeniedReason::PasswordExpired)),
        "kevin" => Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: "Call Human Resources!".to_string() })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
```

Now we want to create a `struct` to represent the import pieces of user information:

```rust
pub struct User {
    username: String,
    password: String,
    action: LoginAction,
}
```

So far, the syntax is just like it was for an enumeration. Let's add a constructor:

```rust
impl User {
    pub fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            action
        }
    }
}
```

Again, the syntax is the same. Two things to note:

* We're using `.to_string()` to convert `&str` into a `String`.
* `action` is the same as the parameter, so you can omit `action: action` (handy shorthand).

## Arrays and Slices

Having just the one user isn't very scalable either, so we'll store a few users in an *array*. Arrays are sequential blocks of *stack* memory (meaning you have limited space available, but it's very fast) that have an *exact* size.

Global variables are deliberately [tricky in Rust](/day3/hour1/globals.md), so let's make a function that returns a list of users:

```rust
pub fn get_users() -> [User; 3] {
    [
        User::new("herbert", "password", LoginAction::Accept(Role::Admin)),
        User::new("bob", "password", LoginAction::Accept(Role::User)),
        User::new("fred", "password", LoginAction::Denied(DeniedReason::PasswordExpired)),
    ]
}
```

That's not bad - you can add users by updating the number of users (the 3), and listing them. Now let's write a function to test a login. Remove the `login` function and replace it with:

```rust
pub fn login(users: &[User], username: &str, password: &str) -> Option<LoginAction> {
    let username = username.trim().to_lowercase();
    let password = password.trim();
    if let Some(user) = users
        .iter()
        .find(|u| u.username == username && u.password == password) {
        Some(user.action.clone())
    } else {
        None
    }
}
```

This doesn't compile, because `LoginAction` and friends don't support being cloned. Cloning is a deep copy, retaining all interior information. Some types can use `Copy` instead - but anything with a `String` in it can't be copied. Copying is generally faster. We're not going to try and return a *reference* yet, we'll dive into lifetimes later.

We can implement `Clone` on our enumerations by adding:
```rust
#[derive(PartialEq, Debug, Clone)]
```
to each header.

You'll notice that while this compiles, Clippy marks it as an issue! It's written out quite cleanly:
    * Take a *slice* of `User` types. A slice is any set of array, vector or other contiguous structures.
    * Create an *iterator*. Iterators are Rust's easy way of interacting with groups of data---almost every container supports iterators.
    * Call `find` which reads each entry and runs the provided comparison closure function---it returns `None` if none are found, and `Some(user)` if one is located.

The reason this generates a `Clippy` error is that Rust provides an easier way to to do this---it's not obvious at first, lots of people do this, that's why `Clippy` has a lint for it. You can replace the whole `if let` with:

```rust
users
.iter()
.find(|u| u.username == username && u.password == password).map(|user| user.action.clone())
```

With Rust Analyzer, you can press `ctrl` + `period` (`.`) and select "try this". It transforms your function for you:

```rust
pub fn login(users: &[User], username: &str, password: &str) -> Option<LoginAction> {
    let username = username.trim().to_lowercase();
    let password = password.trim();
    users
        .iter()
        .find(|u| u.username == username && u.password == password).map(|user| user.action.clone())
}
```

## Let's test it

> This session is live coded. The GitHub repo entry is [here](/src/auth_struct_exe/)

We can make a few simple changes to `login`:

```rust
use auth_struct::*;

fn user_accepted(role: &Role) {
    println!("You are logged in as a {role:?}");
}

fn main() {
    let users = get_users();

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
```

To note:
* We fetch the user list with `get_users()`.
* We retrieve both a username and a password.
* When we call `login`, we can automatically convert the users array into a slice by borrowing it.
* The rest of the program is unchanged - but now we can easily change the user list.