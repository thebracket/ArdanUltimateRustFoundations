# Ultimate Rust 1: Foundations

This is a four day class. Each day is 4 hours, with a short break at the end
of each hour.

## Class Overview

| **[Day 1](./day1/readme.md)**                        | **Day 2**                         | **Day 3**                     | **Day 4**         |
|------------------------------------------------------|-----------------------------------|-------------------------------|-------------------|
| [Introduction](./day1/class_intro.md#class-overview)            |
| [Setup & Update Rust](./day1/setup_rust.md)                     | Handling Errors                   | New Types and Wrapping Data   | Simple Benchmarks |
| [Visual Studio Code](./day1/setup_ide.md)                       | `thiserror`                       | Traits: `From`/`Into`         | Feature Flags     |
| [cargo init and "Hello World"](./day1/hello_world.md)           | `anyhow`                          | Dynamic Traits and Pseudo-OOP | Macros            |
| [Cargo Workspaces](./day1/workspaces.md)                        | Single-Threaded Borrow Checker    | Combining Traits              | Safety Guarantees |
| [Hello Library](./day1/hello_library.md)                        | Lifetimes                         | Casting with `any`            |                   |
| [Text Input & Better Unit Testing](./day1/simple_login_test.md) | RAII - Drop Cleanup               | Generic Functions             |                   |
| Enums & Options                            | Fearless Concurrency              | Generic Data Types            |                   |
| Structures                                 | Rust Saves the Day                |                               |                   |
| Arrays & Iterators                         | Thread-shared Atomics             |                               |                   |
| Vectors                                    | Using `rayon` for easy threading  |                               |                   |
| HashMaps                                   | TCP Server with Tokio             |                               |                   |
| Dependencies                               | Green Threads and Blocking        |                               |                   |
| Serialization                              | Sending Receiving data - Channels |                               |                   |
| Salted Hashes & Password                   |                                   |                               |                   |
| Serialization to Other Formats             |                                   |                               |                   |
| User Manager CLI Application               |                                   |                               |                   |
| Clap for CLI Options                       |                                   |                               |                   |


## Day 2: Fearless Concurrency

### Hour 1 

* Day Intro
* Hour Intro
* Dealing with Errors
    * Using `thiserror` to be really specific.
    * Using `anyhow` for easy client-side handling.
    * Errors are not exceptions.
* The Borrow Checker - Single Threaded
    * The borrow checker rules
        * You can borrow something immutably as often as you like.
        * You can only borrow something that can change once - and not if anyone is looking at it.
    * How does this help?
    * No use after move.
    * The Borrow Checker Cycle for C programmers
    * Lifetimes
* Guaranteed Cleanup with Drop
* Hour Wrap
* Break

### Hour 2

* Hour Intro
* Let's create a really inefficient prime number checker, and count primes.
    * And of course, unit test it.
* Take a quick look at the C++ example of how this can go horribly wrong.
* Let's divide the workload with vector "chunks".
* Now, let's read each "chunk" in a thread.
    * Rust's borrow checker and data race protection makes it impossible to unsafely share the counter!
    * So let's use an `AtomicUsize` with ordering to safely increment as the threads go by.
    * It's a lot faster.
* Hour Wrap
* Break

### Hour 3

* Hour Intro
* Let's rewrite the checker to use iterators.
    * Much shorter.
    * Iterators are often the preferred way to interact with data.
* Now let's add `rayon` and use `par_iter`.
    * We just turned the whole program into a parallel system that uses all of our CPU cores - with 1 line of code.
* Rayon is great for CPU bound applications.
* Not every problem is CPU bound. Sometimes, async is a better choice.
    * `Tokio` simple async TCP echo server.
    * Connect Tokio loop to read to end, deserialize the request and return a serialized result. Use the login example again.
* Hour Wrap
* Break

### Hour 4

* Hour Intro
* What if you have mostly I/O bound issues, but some CPU bound issues, too?
    * The Tokio sleep example
    * Introducing `spawn_blocking`.
    * Running Rayon and Tokio at once. Prime numbers as a service.
* Communicating with channels
* Hour Wrap
* Break

## Day 3

### Hour 1 

* Day Intro
* Hour Intro
* Using "New Types" to wrap data types.
* First touch of traits: From/Into
* Hour Wrap
* Break

### Hour 2

* Hour Intro
* Traits aren't objects, but you can use them similarly.
    * The infamous "animal" trait.
    * `dyn` and `Box` to store dynamic data.
* Traits that combine other traits.
* Casting with `any`
* Hour Wrap
* Break

### Hour 3

* Hour Intro
* Building generic functions with traits
* Hour Wrap
* Break

### Hour 4

* Hour Intro
* Building generic data types.
* Hour Wrap
* Break

## Day 4

### Hour 1 

* Day Intro
* Hour Intro
* Running simple benchmarks
* Using feature flags to change behavior at compile time
* Hour Wrap
* Break

### Hour 2

* Hour Intro
* Simple macros
* Hour Wrap
* Break

### Hour 3

* Hour Intro
* ?
* Hour Wrap
* Break

### Hour 4

* Hour Intro
* Rust's Safety Guarantees
* Hour Wrap
* Break

Day Intro

----

Part 1: The Rust Ecosystem.
!Rust includes Cargo, a swiss-army knife tool that can:
!Create skeleton applications, with git integration built-in.
!Provide a consistent build environment.
!Execute and manage unit tests.
Benchmark your code.
!Manage dependencies.
Using Rust without Cargo:
If you are including Rust as part of an existing application, you don’t have to use Cargo. rustc can integrate with a Makefile, Cmake or other build environment.
Rust’s Safety Guarantees
Memory Safety
!Data-Race Protection
!The “Borrow Checker”
Practical:
“Hello World” application – the foundation of most languages.
“Hello Library” – build a Rust library in 5 minutes, including unit tests.
Benchmark: track your library’s performance over time.
Dependencies: build an application that uses dependencies with Cargo.

Part 2: Data Management.
!Arrays!, !vectors and !slices.
Strict types and strong typing.
!Serializing and De-Serializing data.
!Iterative Programming – using iterators to combine operations.
!Working with Strings – why are there two types of string, and why is a character a variable-sized type?

Part 3: Fearless Concurrency
!Rust is designed from the ground-up to let you maximize performance, with threads and asynchronous execution patterns.
Protection from data races
!Example C++ program that compiles without warnings, but gives the wrong results because of a data race.
!Example of Rust not permitting an equivalent example to compile: you are protected against the most common concurrency errors out of the box.
!Easy multi-threaded concurrency with Rayon
!Ultimate control with raw threads
!High-performance Input/Output with Asynchronous design and Tokio.
Practicals:
!Build a CPU bound program with Rayon, and use all of your CPU power.
!Manage raw-threads and shared data.
!Spin up a simple server in 10 minutes.

Part 4: Traits, and Generics
!Rust is not an object-oriented language – you can’t inherit functionality from one class into another.
!Rust traits let you define an interface (with default behavior) and provide runtime or compile-time dynamic dispatch.
!Rust generics let you tailor functions and types to the needs of the structure’s consumer.
Practicals:
!The classic Object-Oriented Programming example of different objects printing different text – but with traits.
!Designing a simple generic function with ToString.
!Stacked traits – traits that depend upon one another, combined with generic programming.
