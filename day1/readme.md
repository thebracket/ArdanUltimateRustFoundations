# Day 1: Core Rust

## 1st Hour

* [Class Introduction](./hour1/class_intro.md)
* [Setup and Update Rust](./hour1/setup_rust.md)
* [Configure your IDE](./hour1/setup_ide.md)
* [Cargo Init and Hello World](./hour1/hello_world.md)
* [Cargo Workspaces](./hour1/workspaces.md)
* [Hello Library](./hour1/hello_library.md)
* [Text Input & Better Unit Testing](./hour1/simple_login_test.md)
* [Hour 1 Wrap](./hour1/hour_1_wrap.md)
* **TIME FOR A BREAK**

## 2nd Hour

* [Enumerations](./hour2/enums.md)
* [Structures](./hour2/structs.md)
* [Hour 2 Wrap](./hour2/hour_2_wrap.md)
* **TIME FOR A BREAK**

## 3rd Hour

* Hour Intro
* Introducing Vectors
    * The "everywhere" type in Rust
    * Vectors are just like arrays - but they can change size.
    * The `vec!` macro lets you predefine them just like arrays.
    * Let's use `push` to add an entry.
* If we had lots of users, it would be more efficient to use a map of users.
    * Introduce `HashMap`.
    * Keyed on username, details in value.
    * Now we can replace the `any` with a `get`.
* Serialization
    * Learn how to use Cargo to add `serde` and `serde_json`.
    * Create a program that creates a vector of users, and saves it to `users.json`.
        * Now we're working with files.
        * RAII - closing is automatic.
        * Now we have another Result type to worry about. Unwrap for now. More later.
    * Prove that it works with a round-trip unit test.
    * Adjust the login program to load `users.json` and use that for validation.
* Don't use plain-text passwords!
    * Use cargo to add `sha` support.
    * Change the writer to hash passwords. Add salt.
    * Change the login program to hash with salt and then check that.
* Maybe we don't want JSON - grab another format (TBD) and it's drop-in capable.
* Hour Wrap
* Break

## 4th Hour

* Hour Intro
* Let's make an interactive user manager CLI application.
    * Load the user list.
    * Collect it into a vector.
    * Sort it.
    * Print users to a list
        * But wait, there's more users than we can fit!
        * Introduce paging with slices to display parts of the list.
    * Add an option to delete a user.
    * Add an option to edit a user.
    * Add an option to add a user.
* What if you prefer a CLI command-based app?
    * Introducing `clap` - command line handler.
    * Build a simple user manager with CLI options.
* Hour Wrap
* Break
