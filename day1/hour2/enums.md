# Enumerations

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum/)

You probably want more options than just to say "yes" or "no" to a user. Let's return to the `authentication` workspace member and open `lib.rs`.

Enumerations should be familiar from other languages. Let's create a simple one for logins:

```rust
pub enum LoginAction {
    Admin,
    User,
    Denied,
}
```

> If you're used to C, Java or similar this should look very familiar. Just like C, enumerations like this one are also stored numerically and you can cast a value with `as (number)`.

Now we can create a login function with more options:

```rust
pub fn login(name: &str) -> LoginAction {
    match name.to_lowercase().trim() {
        "herbert" => LoginAction::Admin,
        "bob" | "fred" => LoginAction::User,
        _ => LoginAction::Denied,
    }
}
```

What's new here?
* The function is returning the `enum` type we crated.
* We're using `match` instead of `if` statements. Pattern matching is central to a lot of Rust's expressive power.
* Match statements follow the syntax `(pattern) => expression`.
* We use `|` to mean `or` in match statements. It's `||` in `if` statements!
* The `_` means "default".
* Rust's pattern matching system is *very* powerful and has *many* options. [The Rust Language documentation details more options](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/expressions/match-expr.html)


```rust
#[test]
fn test_enums() {
    assert_eq!(login("Herbert"), LoginAction::Admin);
    assert_eq!(login("bob"), LoginAction::User);
    assert_eq!(login("fred"), LoginAction::User);
    assert_eq!(login("anonymous"), LoginAction::Denied);
}
```

*Oops* - now everything is red. Enumerations don't implement an equality check by default. They also don't support debug printing by default. Fortunately, it's easy to add:

```rust
#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Admin,
    User,
    Denied,
}
```

`#[derive]` can add a lot of implementations to your code automatically. It's even possible to write your own derivations with procedural macros, but that's a more advanced topic than we'll get to today.

> You can expand `derive` macros, too. In this case, it's added some simple code:

```rust
// Recursive expansion of PartialEq! macro
// ========================================

impl core::cmp::PartialEq for LoginAction {}
```

`PartialEq` is a *trait*. Traits are interfaces that describe the capabilities of a type. In this case, you're telling Rust that `LoginAction` can be compared with itself for equality.

With that in place, we can run `cargo test` and our unit test succeeds.

```
running 4 tests
test tests::test_enums ... ok
test tests::test_case_and_trim ... ok
test tests::test_greet_user ... ok
test tests::test_login_fail ... ok
```

As you can see, you can use Rust enumerations just like enumerations in other languages. Alternatively, you can start to more richly express yourself!

## Let's update the login client

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum_exe/)

Open `login` again, and we'll change things around a little:

```rust
use auth_enum::{login, LoginAction};

fn main() {
    println!("Welcome to the (Not Very) Secure Server");
    println!("Enter your username:");
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    match login(&input) {
        LoginAction::Admin => println!("You are administrator"),
        LoginAction::User => println!("You have regular user rights"),
        LoginAction::Denied => println!("You are not allowed in"),
    }
}
```

We're using the new `login` function, and performing a `match` on the result. Just like we did in the unit test.

In a real system, you want to convey more than just a role.

Test this: login as each user in turn.

## Enumerations with Variables

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum2/)

Let's create a new enumeration for user roles:

```rust
#[derive(PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
    Limited
}
```

Let's also create an enumeration for access being denied:

```rust
#[derive(PartialEq, Debug)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked{reason: String},
}
```

What's this? `reason` is attached to `AccountLocked`. This works because Rust enumerations share a lot in common with `union` types in other languages. You can attach variables to individual entries.

> Enumerations with data are the size of the largest member. A `String` occupies two `usize` variables: the length of the string and a pointer to the string data. Don't go too nuts adding complicated data to `enum` types!

Now let's modify our initial `LoginAction` enum to be a bit more specific:

```rust
#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}
```

This is another way to wrap variables in an enum: you can use them as a *tuple*. Each enum entry has an interior type storing either role or denied reason.

Let's update `login` to use these types:

```rust
pub fn login(name: &str) -> LoginAction {
    match name.to_lowercase().trim() {
        "herbert" => LoginAction::Accept(Role::Admin),
        "bob" => LoginAction::Accept(Role::User),
        "fred" => LoginAction::Denied(DeniedReason::PasswordExpired),
        _ => LoginAction::Denied(DeniedReason::AccountLocked { reason: "Not on the list".to_string() })
    }
}
```

"Not on the list" isn't a great reason, but we're going to deal with non-existent users in a moment. Let's update our unit test to work with these options:

```rust
#[test]
fn test_enums() {
    assert_eq!(login("Herbert"), LoginAction::Accept(Role::Admin));
    assert_eq!(login("bob"), LoginAction::Accept(Role::User));
    assert_eq!(login("fred"), LoginAction::Denied(DeniedReason::PasswordExpired));
    assert_eq!(login("anonymous"), LoginAction::Denied(DeniedReason::AccountLocked { reason: "Not on the list".to_string() }));
}
```

Not being on the list isn't necessarily a reason for denial in many systems---it might be an opportunity to sign someone up. Instead of a role, we want to express that there *may be* a user account---or there may not. If there is, it has the characteristics we've described. If there isn't an account, we want to clearly express that.

## Optional Users

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum3/)

Let's extend `login` to include an `Option` - a type that either has or doesn't have a value. It's the closest you'll get in safe Rust to a `NULL`, but forces you to check it---avoiding a lot of the `NULL` problems found in other languages.

```rust
pub fn login(name: &str) -> Option<LoginAction> {
    match name.to_lowercase().trim() {
        "herbert" => Some(LoginAction::Accept(Role::Admin)),
        "bob" => Some(LoginAction::Accept(Role::User)),
        "fred" => Some(LoginAction::Denied(DeniedReason::PasswordExpired)),
        "kevin" => Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: "Call Human Resources!".to_string() })),
        _ => None,
    }
}
```

Now we've changed the return type to `Option<LoginAction>`. That means that *every* response must be `Some(..)` or `None`.

Let's update the unit test to match:

```rust
#[test]
fn test_enums() {
    assert_eq!(login("Herbert"), Some(LoginAction::Accept(Role::Admin)));
    assert_eq!(login("bob"), Some(LoginAction::Accept(Role::User)));
    assert_eq!(login("fred"), Some(LoginAction::Denied(DeniedReason::PasswordExpired)));
    assert_eq!(login("kevin"), Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: "Call Human Resources!".to_string() })));
    assert_eq!(login("anonymous"), None);
}
```

It's pretty tedious to keep typing "Call Human Resources!"---and the message might vary. Let's change the test to use a different structure:

```rust
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
```

`if let` is the same as a `match` statement, with only one pattern to match. If the pattern matches, the inner scope executes - otherwise the `else` runs. It'll make a little more sense in a moment.

Running `cargo test` shows that our code is still in good shape.

## Pattern Matching Login Client

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum_exe3/)

Switch back to the `login` workspace and let's build an expressive `match` to handle all eventualities:

```rust
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

```

There's a lot to process here:
* `match` can handle nested structures. `None` is easy, but `Some(..)` can wrap possible contents.
* You can *capture* variables from inside a statement. We're doing that for `role` and `reason` - just make sure that the variable aligns with the layout of the enumeration.
* You can ignore variables by either marking them `reason: _` or `..` to ignore all of them.
* You can nest `match` statements.
* There's no fall-through: no need to `break`.

And best of all, the compiler will warn you if you add an enumeration and forget to check it. In the live demo, temporarily add another `Role`, and watch the error light up.

## Enumerations can Contain Functions

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum_fn/)

Enumerations can contain complex data, they can also contain functions. There are two major types of function that can be implemented inside an `enum`: associated functions and member functions. Let's start by making a constructor to reduce our typing a bit:

```rust
impl LoginAction {
    fn standard_user() -> Self {
        LoginAction::Accept(Role::User)
    }
}
```

There's altogether too many types of `self` in Rust.
* `Self` (uppercase) refers to the type itself. The constructor returns a variable of its type.
* The function declaration `standard_user()` does *not* contain any reference to `self` - making it an *associated function*.

You can use the constructor to change Bob's creation:

```rust
// WAS:
"bob" => Some(LoginAction::Accept(Role::User)),
// NOW:
"bob" => Some(LoginAction::standard_user()),
```

Since we're only going to be returning standard users who exist, let's clean up one stage further:

```rust
impl LoginAction {
    fn standard_user() -> Option<Self> {
        Some(LoginAction::Accept(Role::User))
    }
}
```

That lets us further simplify the `bob` case to:
```rust
"bob" => LoginAction::standard_user(),
```

Constructors are great for reducing the amount of typing you do. *Associated* functions can interact with the *type* (`Self`), but they can't interact with the content of any particular variable. To do that, you need a `member function`. Let's create a login handler that doesn't assume any of the functionality from the parent project:

```rust
pub fn do_login(&self, on_success: fn(&Role), on_denied: fn(&DeniedReason)) {
    match self {
        Self::Accept(role) => on_success(role),
        Self::Denied(reason) => on_denied(reason),
    }
}
```

There's some new things here:
* `&self` means "provide a read-only reference to myself". The function can see the current value.
* `fn(&Role)` and `fn(&DeniedReason)` are *function pointers*. You can pass in a function as a parameter, and call that function inside the function. This is really useful for separating roles in your project: the enumeration doesn't have to know anything about what you're doing inside those functions, it just has to know how to call them.

## Using `do_login` in the login program

> This session is live coded. The GitHub repo entry is [here](/src/auth_enum_fn_exe/)

Open up the `login` project again. Let's change `main.rs` as follows:

```rust
use auth_enum_fn::*;

fn user_accepted(role: &Role) {
    println!("You are logged in as a {role:?}");
}

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
        Some(login_action) => {
            login_action.do_login(
                user_accepted, 
                |reason| {
                    println!("Access denied!");
                    println!("{reason:?}");
                }
            )
        }
    }
}
```

Things to notice:
* `user_accepted` is a regular function.
* `{:?}` is a format macro shorthand for "print the debug representation"
* The `do_login` function accepted `user_accepted` as a parameter. The enumeration's function receives this as a pointer, and calls back to it.
* For "denied", we used a *closure*. In this case, we're just writing an inline function.
