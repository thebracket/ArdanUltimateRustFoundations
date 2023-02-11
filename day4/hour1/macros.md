# Simple Macros

Macros are Rust's way of letting you change the syntax of the language. There are actually two types of macro:

* Declarative Macros
* Procedural Macros

Procedural macros are insanely powerful, but are more advanced than will fit into this class. Declarative macros are really useful, so we'll learn to use them.

## Changing Syntax

Unlike C, Rust macros are *hygienic*. They won't surprise you:
* Using a macro is always marked with a `!`. No surprises.
* You can't `#DEFINE TRUE FALSE` to ruin someone's day. Macros don't leak outside of their scope.

Have you ever wished that `push` let you add more than one element at a time to a vector? With a declarative macro, it can.

> This is live coded, the Github is [here](/src/macros)

Let's start by dreaming up the syntax we'd like:

```rust
fn main() {
    let mut vec = Vec::new();
    push!(vec, 1, 2, 3, 4);
}
```

So we want a macro that accepts a vector as input, and pushes multiple elements to it.

Let's start a little more simply:

```rust
push!(vec, 1);
```

Start making a macro by typing `macro_rules!`. I recommend letting your IDE fill out the default:

```rust
macro_rules!  {
    () => {
        
    };
}
```

Now we *name* the macro:

```rust
macro_rules! push {
    () => {
        
    };
}
```

The `()` before the `=>` is a *pattern*. Right now, it'll only match `push!()`. We know that we want two parameters. The syntax is slightly different to normal Rust:

```rust
macro_rules! push {
    ($target: expr, $val: expr) => {
        
    };
}
```

You decorate macro variables with a `$`. `expr` is the most common type, representing an `expression`. There are a ton of different ones, including identifiers and keywords.

So now, let's make the macro do something:

```rust
macro_rules! push {
    ($target: expr, $val: expr) => {
        $target.push($val);
    };
}
```

> Clippy helpfully warns us that we could be using the `vec!` macro. Ignore that for now.

Your basic case now works. You can push an entry into a vector with the macro.

So what about multiple entries? You need to use *repeating patterns*:

```rust
($target: expr, $($val: expr),+) => {
```

The `$()+` construct indicates that the contents will *repeat*. Adding `,` indicates that we are separating each element with a comma. You could use a different character if you want, but comma is traditional.

How do we repeat inside the macro?

```rust
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            $target.push($val);
        )+
    };
}
```

We wrap the repeating part in another `$(` and `)+`.

Go back to main and enable your original syntax:

```rust
fn main() {
    let mut vec = Vec::new();
    push!(vec, 1, 2, 3, 4);
    println!("{:?}", vec);
}
```

Run the program:

```
[1, 2, 3, 4]
```

It worked! You've built your very own `push!` macro.

If you use `Expand Macro` from Rust Analyzer, you can see that the macro expands into individual calls:

```rust
// Recursive expansion of push! macro
// ===================================

vec.push(1);
vec.push(2);
vec.push(3);
vec.push(4);
```

That's how procedural macros work: they run as a pre-processor before compilation, and expand the source-code in-place.

If you want to export macros for other programs, add:

```rust
#[macro_export]
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            $target.push($val);
        )+
    };
}
```

The macro will *always* end up at the root of your crate.

> A word of warning. Macros can take longer to compile.

## Recursion

Macros can even call other macros.

```rust
macro_rules! really_push {
    ($target: expr, $val: expr) => {
        $target.push($val);
    };
}

#[macro_export]
macro_rules! push {
    ($target: expr, $($val: expr),+) => {
        $(
            really_push!($target, $val);
        )+
    };
}
```
