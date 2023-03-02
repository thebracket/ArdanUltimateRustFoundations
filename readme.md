# Ultimate Rust 1: Foundations

![](/images/ardanlabs-logo.png)

Presented by [Ardan Labs](https://www.ardanlabs.com/), Ultima Rust: Foundations gives you a "zero to hero" class to get you started with Rust. You'll learn the basics of Rust, before diving into hands-on learning that can help you build a Rust foundation into your architecture. Take advantage of the speed and safety of Rust, tame the borrow and lifetime checkers, and pick up some tricks to help you become productive in Rust.

This is a four day class. Each day is 4 hours, with a short break at the end of each hour.

Classes are presented in a live-coding workshop environment. You are encouraged to code along, and keep this guide as a road map into Rust.

## Pre-Requisites

You need:

* Rust installed (via RustUp.rs)
* An IDE configured to use Rust Analyzer.
    * The instructor will use Visual Studio Code, so if you want to match the examples exactly with what you see, this is the recommended platform. It is [available free](https://code.visualstudio.com/download) for Mac, Windows and Linux.

You should also clone a copy of these notes for yourself:

```
git clone https://github.com/thebracket/ArdanUltimateRustFoundations.git
```

## Class Overview

**[Day 1](/day1/)**

* [Introduction](./day1/hour1/class_intro.md#class-overview)
* [Setup & Update Rust](./day1/hour1/setup_rust.md)
* [Visual Studio Code](./day1/hour1/setup_ide.md)
* [cargo init and "Hello World"](./day1/hour1/hello_world.md)
* [Cargo Workspaces](./day1/hour1/workspaces.md)
* [Hello Library](./day1/hour1/hello_library.md)
* [Text Input & Better Unit Testing](./day1/hour1/simple_login_test.md)
* [Enumerations/Unions](./day1/hour2/enums.md)
* [Structures](./day1/hour2/structs.md)
* [Arrays & Iterators](./day1/hour2/structs.md)
* [Vectors](./day1/hour3/vectors.md)
* [HashMaps](./day1/hour3/hashmaps.md)
* [Serialization](./day1/hour3/serialization.md)
* [Hashing Passwords](./day1/hour3/hashing.md)
* [A CLI Application](./day1/hour4/cli.md)

**[Day 2](/day2/)**

* [Modules & Visibility](./day2/hour1/modules.md)
* [Documentation](./day2/hour1/documentation.md)
* [Error Handling](./day2/hour1/errors.md)
* [The Borrow Checker](./day2/hour2/borrow_checker.md)
* [Lifetimes](./day2/hour2/lifetimes.md)
* [OOP Patterns](/day2/hour2/oop.md)
* [RAII - Drop Cleanup](./day2/hour2/raii.md)
* [CPU Bound Workload](./day2/hour3/count_primes.md)
* [Racing Data](./day2/hour3/data_race.md)
* [Atomically Counting Primes](./day2/hour3/atomic.md)
* [Shared Data](./day2/hour3/shared.md)

**[Day 3](/day3/)**

* [Easy Concurrency with Rayon](./day2/hour3/rayon.md)
* [Async with Tokio](./day2/hour4/tokio.md)
* [TCP Server with Tokio](./day2/hour4/tcp_server.md)
* [Sending Receiving data - Channels](./day2/hour4/channels.md)
* [Global Variables](./day3/hour1/globals.md)
* [Synchronization Primitives](./day3/hour1/sync.md)
* [Type Safety with NewTypes](./day3/hour1/new_types.md)
* [Basic Traits](./day3/hour1/traits.md)
* [Generic Data Types](./day3/hour2/generic_data.md)
* [More Complex Generic Data](/day3/hour2/generic_complex.md)
* [Constants & Constant Functions](./day3/hour2/constants.md)
* [Macros](./day4/hour1/macros.md)
* [Feature Flags](./day4/hour1/feature_flags.md)

**[Day 4](/day4/)**

* [Simpler Locks: Parking Lot](./day4/hour1/parking_lot.md)
* [Avoiding Locks with DashMap](./day4/hour1/dashmap.md)
* [Simple Benchmarks](./day4/hour1/benchmarks.md)
* [Complex Benchmarks - A Quick Visit](./day4/hour1/benchmarks2.md)
* [Putting it together: a TCP login server](./day4/hour1/tcp_login.md)
* [A Simple Web Application](./day4/hour1/rocket.md)
* [How fast are our network services?](./day4/hour1/netbench.md)
* [Life without the Standard Library](./day4/hour1/nostd.md)
* [Rust's Safety Guarantees](./day4/hour1/safety.md)
* Wrap-Up
