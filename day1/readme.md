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

* Hour Intro
    * Introducing enums & Options
        * Create an enum named Action
        * If a user is recognized, return Some(action).
        * If a user is not recognized, return None
        * Match on enum - Eq and PartialEq
        * Enums can contain data!
            * Add a note field to an entry
            * Enums are like unions in other language: they are as large as the largest entry.
        * Match on an enum with fields
        * Maybe you prefer a more object-like syntax
            * You can implement code on enums.
            * Build a function that matches on self to perform differently
    * Introducing structures
        * Build a "User" structure, containing the name, password and action.
            * Side track into string types.
                * &str - for constant strings.
                * `String` for strings that might change.
        * Implement a function on the structure to check username and password, and return Option<Action>. Implement the password check.
    * Introduce arrays & iterators
        * Make a static array of users.
        * Write a simple "for" loop that checks each record.
            * Notice how Clippy complained?
        * First try: we'll use `enumerate` to get the index and do a `for user in users.iter()`.
        * Second try: we'll use `.iter().find`.
            * This introduces closures.
        * Final: we'll use `any`.
* Hour Wrap
* Break

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
