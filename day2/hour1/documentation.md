# Documentation

> This will be a very quick section. It's live-coded, you can find the Github version [here](/src/docs/) and [here](/src/docs_exe)

Building a library implies that you are sharing it--or intend to reuse it later. It's a great idea to include some documentation! I'm not going to make you sit and write documentation with me, that would be pretty dull. It is important that you know what's available, though.

Open up `lib.rs`, and we'll start with a *library header*. At the top, add:

```rust
//! # Authentication
//! 
//! This module provides user validation and role assignment.
```

Now open up `main.rs` in `login`. Hover the mouse over `use authentication`, and you can see your module header:

![](/images/ModuleDocs.png)

Ideally, you want to document everything that is publicly available. We won't do that today, we'd be here all morning. Let's tell Clippy to fire a warning for everything we *should* document. In `lib.rs`, add the following to the top:

```rust
#![warn(missing_docs)]
```

* The `#!` means "do this for the whole crate, underneath this point.

Notice that lots of warnings appeared!

Let's document `hash_password`:

```rust
/// `hash_password` applies a SHA256 digest hash to a string, and returns a
/// string containing the hashed string, in hexadecimal format.
/// 
/// ## Arguments
/// 
/// * `password` - the password to hash.
pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}
```

Notice that we've used markdown. You can even use links if you want to. If you find a usage of `hash_password` and mouse over it, your documentation is now visible:

![](/images/FunctionDocs.png)

## Documentation Examples

Let's make that documentation a little better by adding a usage example:

```rust
/// ```
/// use docs::hash_password;
/// println!("{}", hash_password("test"));
/// ```
```

Save the file, and run `cargo test`. The unit test executed your example! This is a great way to make sure that your docs stay synced with your code.

We'll remove the warning directive so we don't see warnings as we continue learning.