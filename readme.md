# Ultimate Rust 1: Foundations

This is a four day class. Each day is 4 hours, with a short break at the end
of each hour.

## Class Overview

| **[Day 1](./day1/readme.md)**                                         | **[Day 2](./day2/readme.md)**                         | **Day 3**                     | **Day 4**         |
|-----------------------------------------------------------------------|-------------------------------------------------------|-------------------------------|-------------------|
| [Introduction](./day1/hour1/class_intro.md#class-overview)            | [Modules & Visibility](./day2/hour1/modules.md)       |
| [Setup & Update Rust](./day1/hour1/setup_rust.md)                     | [Documentation](./day2/hour1/documentation.md)        | New Types and Wrapping Data   | Simple Benchmarks |
| [Visual Studio Code](./day1/hour1/setup_ide.md)                       | [Error Handling](./day2/hour1/errors.md)              | Traits: `From`/`Into`         | Feature Flags     |
| [cargo init and "Hello World"](./day1/hour1/hello_world.md)           | [The Borrow Checker](./day2/hour2/borrow_checker.md)  | Dynamic Traits and Pseudo-OOP | Macros            |
| [Cargo Workspaces](./day1/hour1/workspaces.md)                        | [Lifetimes](./day2/hour2/lifetimes.md)                | Combining Traits              | Safety Guarantees |
| [Hello Library](./day1/hour1/hello_library.md)                        | [RAII - Drop Cleanup](./day2/hour2/raii.md)           | Casting with `any`            |                   |
| [Text Input & Better Unit Testing](./day1/hour1/simple_login_test.md) | [CPU Bound Workload](./day2/hour3/count_primes.md)    | Generic Functions             |                   |
| [Enumerations/Unions](./day1/hour2/enums.md)                          | [Racing Data](./day2/hour3/data_race.md)              | Generic Data Types            |                   |
| [Structures](./day1/hour2/structs.md)                                 | [Atomically Counting Primes](./day2/hour3/atomic.md)  |                               |                   |
| [Arrays & Iterators](./day1/hour2/structs.md)                         | [Easy Concurrency with Rayon](./day2/hour3/rayon.md)  |                               |                   |
| [Vectors](./day1/hour3/vectors.md)                                    | [Async with Tokio](./day2/hour4/tokio.md)             |                               |                   |
| [HashMaps](./day1/hour3/hashmaps.md)                                  | [TCP Server with Tokio](./day2/hour4/tcp_server.md)   |                               |                   |
| [Serialization](./day1/hour3/serialization.md)                        | [Sending Receiving data - Channels](./day2/hour4/channels.md) |                       |                   |
| [Hashing Passwords](./day1/hour3/hashing.md)                          |                                                       |                               |                   |
| [A CLI Application](./day1/hour4/cli.md)                              |                                                       |                               |                   |


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
