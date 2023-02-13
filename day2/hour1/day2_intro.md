# Day 2

Yesterday, we learned a lot of the basics of Rust:
* Workspaces, applications and libraries.
* Importing dependencies.
* Enumerations
* Structures
* Arrays
* Vectors
* HashMaps
* Serialization
* Hashing Passwords
* A Full CLI application using `clap`

Today, we're going to start putting our knowledge of the basic language to work---and being productive. We'll start by covering some of the sticking points people encounter when getting started with Rust:

* Modules, Visibility and Organizing Your Code.
* Error Handling
* The Borrow Checker
* Lifetimes
* Resource Acquisition is Initialization

Then, we're going to dive into Rust's promises of *Fearless Concurrency*.

* Build a CPU-bound workload.
* Take a look at a couple of other languages that have race condition problems.
* Build the same program in Rust, and see how Rust saves our bacon.
* Use *atomics* for thread-safe primitive variables.
* Use `Rayon` to introduce really easy parallel programming for CPU-bound workloads.
* Use `async` and `Tokio` to build a TCP networking server.

At the end of today, you'll be able to write simple network services that use Rust to safely share data between processes, and divide your CPU-bound workloads into bite-sized chunks---using all the available CPU with limited effort.

