# Feature Flags

Sometimes, you want library consumers to be able to change the behavior of your library. You've used this when you specified `features = [ "derive" ]` with Serde and other crates.

> This is live-coded. The Github version uses [this library]() and [this client]()

Create two new applications:

```
cargo new feature_lib --lib
cargo new feature_exe
```

Let's start with `flags_lib/Cargo.toml`:

```toml
[package]
name = "flags_lib"
version = "0.1.0"
edition = "2021"

[features]
default = [ "normal" ]
normal = []
other = []

[dependencies]
```

We've created a `normal` and `other` feature. The `[]` indicates that they don't turn on other features. `default` indicates that if you don't specify defaults, `normal` will be active.

Now open `flags_lib/lib.rs`:

```rust
pub const MODE: &str = "NORMAL";
pub const MODE: &str = "OTHER";
```

Defining a constant and redefining it doesn't work. But you can see what we're trying to do. Let's require features to enable this constant:

```rust
#[cfg(feature = "normal")]
pub const MODE: &str = "NORMAL";

#[cfg(feature = "other")]
pub const MODE: &str = "OTHER";
```

Now if feature is "normal", `MODE` will equal `NORMAL`. If "other" is enabled, mode will equal `OTHER`.

Open up `flags_exe\Cargo.toml`:

```toml
[package]
name = "flags_exe"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
flags_lib = { path = "../flags_lib" }
```

And the `main.rs` file:

```rust
use flags_lib::MODE;

fn main() {
    println!("{MODE}");
}
```

If we `cargo run`, we see:

```
NORMAL
```

Now edit `Cargo.toml` again:

```toml
flags_lib = { path = "../flags_lib", default_features = false, features = [ "other" ] }
```

Running the program shows `OTHER`.

## Feature Flag Conditionals

You'll also notice that VSCode has lit up `lib.rs` in red. You can't define a constant twice, and it's possible to leave `default_features` enabled and *also* enable `other`.

So we add conditions to our feature flags:

```rust
#[cfg(all(not(feature = "other"), feature = "normal"))]
pub const MODE: &str = "NORMAL";

#[cfg(all(not(feature = "normal"), feature = "other"))]
pub const MODE: &str = "OTHER";
```

Unfortunately, you need to start using external crates to make this cleaner.