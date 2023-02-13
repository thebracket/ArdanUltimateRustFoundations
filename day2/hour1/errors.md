# Error-Handling

So far, we've largely ignored errors. Whenever something might return an error, we've called `unwrap` or the slightly better `expect`. Both of these are the nuclear option for error-handling: when you encounter an error, crash the whole program.

* `unwrap` crashes with only a stack trace.
* `expect("Something bad happens")` crashes with the error message and a stack trace.

You probably want your program to at least try and gracefully handle errors.

> This is live-coded. The GitHub version is [here](/src/errors1/)

Let's create a new project with `cargo init errors` and adding it to our workspace. We're going to use it as a playground for error-handling.

Let's make a new function and call it:

```rust
fn get_line_from_keyboard() -> String {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();
    let trimmed = input.trim();
    trimmed.to_string()
}

fn main() {
    println!("Enter your name:");
    let name = get_line_from_keyboard();
    println!("{name}");
}
```

Very straightforward, and it has the dreaded `unwrap`! Looking at `read_line`, it returns an `io::Result`.

Let's change our function to return a `Result` type too, using the error from the function we're calling:

```rust
fn get_line_from_keyboard() -> Result<String, std::io::Error> {
```

We also have to change the return from `input` to `Ok(input)`---we're responding that "no error occurred, it's ok, here's the data". We can now replace the `unwrap` with a `?`. And in `main`, let's debug-print `name` - it's a Result type now. Here's the complete program:

```rust
fn get_line_from_keyboard() -> Result<String, std::io::Error> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}

fn main() {
    println!("Enter your name:");
    let name = get_line_from_keyboard();
    println!("{name:?}");
}
```

So that's great, if an error occurs - we know about it and print that something bad happened. Now let's ask the user to enter an integer.

```rust
fn get_int_from_keyboard() -> Result<i32, std::io::Error> {
    let text = get_line_from_keyboard()?;
    Ok(text.parse()?)
}
```

Oh no! `parse()` *also* returns errors, but not of the same type. So now you have a problem returning an `std::io::Error`---because `parse` generates a different error. The canonical way around this is to create a *dynamic* Rust type---a pointer to a variable containing something that implements a trait named `Error`. You'll learn more about this tomorrow.

So go ahead and declare a type:

```rust
use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
```

And change your functions to read:

```rust
fn get_line_from_keyboard() -> Result<String> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}

fn get_int_from_keyboard() -> Result<i32> {
    let text = get_line_from_keyboard()?;
    Ok(text.parse()?)
}
```

Let's change the body to use the new function:

```rust
fn main() {
    println!("Enter an integer:");
    let number = get_int_from_keyboard();
    println!("{number:?}");
}
```

Let's run this a couple of times and see what we get:

```
Enter an integer:
5
Ok(5)

Enter an integer:
blah
Err(ParseIntError { kind: InvalidDigit })
```

## Handling Errors

With boxed errors, you've reduced your access to the inner error. It's hard to `match` on what went wrong:

```rust
fn main() {
    loop {
        println!("Enter an integer:");
        let number = get_int_from_keyboard();
        match number {
            Ok(n) => { println!("You entered {n}"); break; },
            Err(e) => println!("Error: {e:?}"),
        }
    }
}
```

This program is probably fine, but there's no distinguishing between a string not being an integer---and not being able to obtain a string at all. The latter is probably worth panicking, the former just requires that you ask the user to try again.

### Consuming Errors with Anyhow

> This is live-coded, the Github version is [here](/src/errors2/)

The `anyhow` crate is a popular choice for *applications*---on the receiving end of errors. It simplifies error handling quite a bit. Let's add `anyhow` to our `Cargo.toml`:

```toml
[dependencies]
anyhow = "1"
```

And modify our program to use it. Remove the boxed Error with:

```rust
use anyhow::Result;
```

Anyhow's `Result` type is very similar to the boxed error we used. It adds a `root_cause()` function for finding the underlying problem. It also provides a rather unwieldy syntax for accessing the error:

```rust
fn main() {
    loop {
        println!("Enter an integer:");
        let number = get_int_from_keyboard();
        match number {
            Ok(n)  => { println!("You entered {n}"); break; },
            Err(e) => {
                if let Some(std::io::Error { .. }) = e.downcast_ref::<std::io::Error>() {
                    panic!("stdin is unavailable");
                } else {
                    println!("Try again - that wasn't an integer");
                }
            }
        }
    }
}
```

That's better---but not exactly ergonomic.

**I recommend using anyhow in client programs when you aren't too worried about exactly what went wrong.**

### Creating Better Errors

> This is live-coded, the Github version is [here](/src/errors3/)

When you're making libraries, you want to provide really specific error messages. This means a bit more typing, but it makes control flow and error handling *much* nicer.

Defining your own errors in pure Rust is very verbose. Instead, pretty much everyone now uses the `thiserror` crate. Replace `anyhow` with it in your `Cargo.toml`:

```toml
[dependencies]
thiserror = "1"
```

Over in `main.rs`, let's start by making ourselves an error type:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum InputError {
    #[error("Standard input is unavailable")]
    StdIn,

    #[error("Cannot parse integer from text")]
    NotAnInteger,
}
```

That very clearly expresses our intentions. Now let's update the program to use it.

```rust
fn get_line_from_keyboard() -> Result<String, InputError> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).map_err(|_| InputError::StdIn)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}
```

Notice:
* We've changed the return type to specify our error type.
* We're using `map_err` on the error to convert an error. The closure is passed the error, but we're ignoring it (`_`).

Let's do the same for the integer input:

```rust
fn get_int_from_keyboard() -> Result<i32, InputError> {
    let text = get_line_from_keyboard()?;
    text.trim().parse().map_err(|_| InputError::NotAnInteger)
}
```

Notice:
* We can use `?` on the `get_line_from_keyboard` because it already returns our error type.
* We once again map an error, this time the parser---to our very own error type.

That lets us really clean up the `main` loop:

```rust
fn main() {
    loop {
        println!("Enter an integer:");
        let number = get_int_from_keyboard();
        match number {
            Ok(n)  => { println!("You entered {n}"); break; },
            Err(InputError::StdIn) => panic!("Input doesn't work"),
            Err(InputError::NotAnInteger) => println!("Please try again"),
        }
    }
}
```

We're explicitly covering every possible error. That's the path to reliable code.

> Discussion: Errors are not exceptions.