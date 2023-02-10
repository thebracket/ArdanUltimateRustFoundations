## Day 2

### Hour 1 

* Day Intro
* [Modules & Visibility](./hour1/modules.md)
* [Documentation](./hour1/documentation.md)
* [Error Handling](./hour1/errors.md)
* Hour wrap
* **TIME FOR A BREAK**

### Hour 2

* [The Borrow Checker](./hour2/borrow_checker.md)
* [Lifetimes](./hour2/lifetimes.md)
* [Resource Acquisition is Initialization](raii.md)
* Hour Wrap

### Hour 3

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