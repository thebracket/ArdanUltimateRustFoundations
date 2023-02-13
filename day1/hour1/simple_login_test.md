# Extending the Library

> This will be live-coded. The GitHub version is [here](/src/auth_if/). You can try it in ReplIt here: [https://replit.com/@HerbertWolverso/SimpleLoginTest#login/src/main.rs](https://replit.com/@HerbertWolverso/SimpleLoginTest#login/src/main.rs)

Just printing someone's name is a great start, but you probably want
some user input.

## Initial Yes/No Authentication

Let's go back to our `auth` project, and add a new function:

```rust
pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "herbert"
}
```

There's a few things to note here:
* We're applying `to_lowercase()` to ensure that even "HeRbErT" is in lower case.
* We're calling `trim()` to remove any extraneous characters from the string.
* We once-again use short-form return. True if the name is "herbert"

Let's also add a couple of unit tests for this:

```rust
#[test]
fn test_case_and_trim() {
    assert!(is_login_allowed("HeRbErT"));
    assert!(is_login_allowed("  herbert\r\n"));
}

#[test]
fn test_login_fail() {
    assert!(!is_login_allowed("bob"));
}
```

The first test covers case and trimming. The second tests the *negative* of
the function---it returns `false` for a username other than `herbert`.

Test it with `cargo test`:

```
running 3 tests
test tests::test_greet_user ... ok
test tests::test_login_fail ... ok
test tests::test_case_and_trim ... ok
```

## Implement Yes/No Login

Now return to your `login` program. Let's change it to require that the user provide their name,
and we determine if they are allowed in or not.

Change `main.rs` to read as follows:

```rust
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
```

What's new here?

* `let mut input = String::new()` creates a new string. It is *mutable* you can change it after it has been created. Rust defaults to immutable variables.
* Obtaining `stdin` gets a link the operating system's `stdin` system.
* `read_line` reads a string. The mysterious `unwrap` function is there because `read_line` can fail. You might not have a keyboard attached, you might be running headless (without an input system at all). `unwrap` means "crash if an error occurs here". We're going to talk more and more about errors and results as we progress.
* We're using an `if` statement to vary what happens for each possible code branch. Since `is_login_allowed` returns a `bool`, it can only be `true` or `false`.

Let's run the program a couple of times (with `cargo run`):

```
Welcome to the (Not Very) Secure Server
bob
Sorry, bob
 is now allowed
```

Alternatively:

```
Welcome to the (Not Very) Secure Server
Enter your username:
herbert
Welcome, herbert
```

Notice the strange line breaks? Reading from `stdin` includes `\r\n` (or just `\n` on non-Windows platforms) in your string. Rust dutifuly sends this to the output, causing a break. Just like you did in the login system, `trim()` can remove this.

> If we're running faster than expected, connect the debugger to demonstrate this.