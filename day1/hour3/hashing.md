# Hashing Passwords

> This session is live-coded. The Github code is [here]()

You probably don't want to be saving passwords in plain text. It's high on the list of "programming oopsies" that lead to security issues.

Open `Cargo.toml` in your `authentication` project. We're going to add one more dependency:

```toml
[package]
name = "auth_passwords"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
serde = { version = "1.0.152", features = [ "derive" ] }
serde_json = "1.0.92"
sha2 = "0"
```

Now open `lib.rs` and we'll create a function to hash passwords:

```rust
pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}
```

The `{:X}` in `format!` means "print in hexadecimal format".

> Add salt! Normally, you wouldn't just use a direct hash - you'd manipulate `password` in some way. There are whole books and papers written on what makes for a good password salting solution. It's a bit out of scope for the class, so we'll keep it simple.

## Hashing New User Passwords

Now lets update the `User` constructor (`new`) to automatically hash the passwords it is given:

```rust
impl User {
    pub fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(),
            password: hash_password(password),
            action
        }
    }
}
```

Likewise, let's have the `login` function hash incoming passwords before comparison is performed:

```rust
pub fn login(users: &HashMap<String, User>, username: &str, password: &str) -> Option<LoginAction> {
    let username = username.trim().to_lowercase();
    let password = hash_password(password.trim());

    users
        .get(&username)
        .filter(|user| user.password == password)
        .map(|user| user.action.clone())
}
```

## Using More Secure Passwords

Not switch over to `login`, and uncomment out `build_users_file()` again. Run the program, quit out and have a look at `users.json`:

```json
{
  "bob": {
    "username": "bob",
    "password": "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8",
    "action": {
      "Accept": "User"
    }
  },
  "fred": {
    "username": "fred",
    "password": "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8",
    "action": {
      "Denied": "PasswordExpired"
    }
  },
  "herbert": {
    "username": "herbert",
    "password": "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8",
    "action": {
      "Accept": "Admin"
    }
  }
}
```

That's much harder to guess! Comment out `build_users_file()` once again, and test that you can still login. You can!