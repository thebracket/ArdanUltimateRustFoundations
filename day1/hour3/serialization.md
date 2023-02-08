# Serialization/Deserialization

It's pretty rare to want to type all of your users into your source code and recompile! It's much more common for the user list to come from an external data source. That might be a database, a file, even a web service. Let's learn to read and write our data in JSON, using serialization.

> This is live-coded. The GitHub example code is [here](/src/auth_json/)

## Dependencies

There's a wonderful Rust package called `serde` for handling serialization/deserialization (where it gets its name). It is format-agnostic, and needs an additional dependency for whatever format you require. In this case, we'll use `serde_json`.

Navigate to your `authentication` project, and open `Cargo.toml`. Now in your command prompt/terminal, run the following:

```
cargo search serde
```

This gives quite the list:

```
serde = "1.0.152"                       # A generic serialization/deserialization framework
discord_typed_interactions = "0.1.0"    # suppose you're working with discord slash commands and you want statically typed reques…
serde_json_experimental = "0.0.0"       # A JSON serialization file format
serde_valid = "0.13.0"                  # JSON Schema based validation tool using with serde.
alt_serde_json = "1.0.61"               # A JSON serialization file format
serde_json = "1.0.92"                   # A JSON serialization file format
serde_partiql = "1.1.65"                # A PartiQL data model serialization file format
deserr = "0.4.0"                        # Deserialization library with focus on error handling
serde-encrypt = "0.7.0"                 # Encrypts all the Serialize
serde-encrypt-core = "0.7.0"            # Encrypts all the Serialize
... and 4131 crates more (use --limit N to see more)
```

There really are over 4,000 crates that make use of Serde in some way. The one you want is at the top. You can either copy the "serde" line into the `[dependencies]` section of `Cargo.toml`, or you can (if you updated to a recent Rust) use `cargo add`:

```
cargo add serde
```

You'll see the following output:

```
    Updating crates.io index
      Adding serde v1.0.152 to dependencies.
             Features:
             + std
             - alloc
             - derive
             - rc
             - serde_derive
             - unstable
```

And if you look at your `Cargo.toml`, it now lists a dependency:

```toml
[dependencies]
serde = "1.0.152"
```

Repeat this for `serde_json`:

```
cargo add serde_json
```

Your `Cargo.toml` file now looks like this:

```toml
[package]
name = "auth_json"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = "1.0.152"
serde_json = "1.0.92"
```

You'll notice that `cargo add` mentioned some "features". Features are *optional* additions to crates that provide additional functionality. In this case, we actually want one of the features of Serde: `derive`. The derive feature adds some procedural macros that makes it really easy to serialize and deserialize known types. So let's edit `Cargo.toml` to specify the feature we want:

```toml
serde = { version = "1.0.152", features = [ "derive" ] }
```

## Making Types Serializable & Deserializable

Open up `lib.rs` (in the `authentication` library). At the top, add a `use` statement:

```rust
use serde::{Serialize, Deserialize};
```

These are derivable macros. You can use them in `#[derive(...)]` statements. Let's make `User` serializable and deserializable:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub action: LoginAction,
}
```

That was too easy---literally, because it didn't work. In order to use the `Serialize` and `Deserialize` macros, every type contained within the structure *also* has to support `Serialize` and `Deserialize`. Let's go ahead and add the derivations to our other types:

```rust
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    Limited
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked{reason: String},
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}
```

That compiled - ship it! Seriously, that is all that's required to map types to being serializable or deserializable. All that remains is to save or load some data.

## Making Some Initial Data

Let's create a new function that creates some initial data:

```rust
pub fn build_users_file() {
    use std::io::Write;

    let users = get_users();
    let json = serde_json::to_string(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}
```

The bulk of this function is handling writing to a file: obtaining access to the `Write` trait (required by `write_all`), creating a file and using `write_all` to write to it. Notice that you never close the file: you don't need to. `File` uses "RAII" - Resource Allocation Is Initialization, and store the file handle internally. When `File` drops out of scope, the file is closed. You'll learn to do this for your types tomorrow.

Also, don't worry about all the unwrapping---that's on the menu for tomorrow, too.

The serialization portion is rather straightforward:

```rust
let json = serde_json::to_string(&users).unwrap();
```

Now open `login` and add a one-liner to the first line in `main()`:

```rust
authentication::build_users_file();
```

Run the program and exit out. In your `login` directory you now have a file named `users.json`. It looks like this:

```json
{"bob":{"username":"bob","password":"password","action":{"Accept":"User"}},"herbert":{"username":"herbert","password":"password","action":{"Accept":"Admin"}},"fred":{"username":"fred","password":"password","action":{"Denied":"PasswordExpired"}}}
```

All of your data is there - but it's not pretty! Let's enable prettiness. In the `build_users_file()` function:

```rust
    //let json = serde_json::to_string(&users).unwrap();
    let json = serde_json::to_string_pretty(&users).unwrap();
```

Run `login` again, and you have a nicely formatted JSON file:

```json
{
  "bob": {
    "username": "bob",
    "password": "password",
    "action": {
      "Accept": "User"
    }
  },
  "herbert": {
    "username": "herbert",
    "password": "password",
    "action": {
      "Accept": "Admin"
    }
  },
  "fred": {
    "username": "fred",
    "password": "password",
    "action": {
      "Denied": "PasswordExpired"
    }
  }
}
```

> Whether or not you want prettiness is up to you. We're going to use the pretty version because it's easier to read. You save disk space and processing time by not formatting the JSON nicely with whitespace.

That really is all you need to create some JSON. How about reading it back?

## Deserializing Data

Now that we have an initial `users.json` file, we don't need to keep recreating it. We'll keep the `build_users` function around just in case, but one-off functions to create some initial data are pretty common.

Let's change `get_users` to load our data from disk. `Serde` really does make it easy. The whole function can be replaced with:

```rust
pub fn get_users() -> HashMap<String, User> {
    let json = std::fs::read_to_string("users.json").unwrap();
    serde_json::from_str(&json).unwrap()
}
```

This loads "users.json" into a `String`, and then deserializes it straight into a `HashMap`.

Let's open the `login` project again.

> This is still live-coding, the Github Example is [here](/src/auth_json_exe)

Comment out the call you made to `build_users_file()` and run the program. It works!

To prove that it really works, let's edit `users.json`. Replace "fred" with "toby":

```json
  "toby": {
    "username": "toby",
    "password": "password",
    "action": {
      "Denied": "PasswordExpired"
    }
  }
```

Now try to login as "toby":

```
Welcome to the (Not Very) Secure Server
Enter your username:
toby
Enter your password:
password
Access denied
PasswordExpired
```

Rust's serialization and deserialization support was one of the features that first sold me on the language. It really is great.