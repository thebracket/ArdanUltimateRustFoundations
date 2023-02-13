# Day 1 Wrap-Up

You've learned a *lot* on your first day:

* You setup and updated Rust.
* You setup Visual Studio Code for Rust development.
* You progressed through "hello world" and "hello library".
* You combined projects with workspaces.
* You've learned the power of Rust's enumeration and pattern matching system.
* You've grouped data together in structures, and implemented functions for structures and enumerations.
* You learned about arrays and iterators, vectors and hashmaps.
* You serialized and de-serialized data with `serde`, in JSON format.
* You learned to use the `sha2` crate to hash passwords.
* You used `clap` to build a full-scale command-line program for managing your user file.

That's enough Rust to get you up and running on a lot of projects.

Tomorrow, we're going to talk about:

* Modules & Visibility
* Documentation
* Error handling, and dealing with `unwrap`.
* The borrow checker and lifetimes.
* RAII - Resource Acquisition is Allocation.
* Fearless Concurrency:
    * How Rust protects you from Data Races
    * Divide work into threads
    * Use Rayon to quickly parallelize CPU-heavy tasks
* Asynchronous Concurrency:
    * Using `tokio` to make a TCP server.
    * Green threads and blocking
* Sending and receiving data with channels.

*Have a good night, and see you tomorrow.*