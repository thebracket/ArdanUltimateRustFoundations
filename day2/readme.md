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

* [Really Inefficiently Counting Prime Numbers](./hour3/count_primes.md)
* [Data Race Time](./hour3/data_race.md)
* [Atomically Counting Primes](./hour3/atomic.md)
* [Easy Parallelism with Rayon](./hour3/rayon.md)
* Hour Wrap

### Hour 4

* [Tokio and Async](./hour4/tokio.md)
* [TCP Networking with Tokio](./hour4/tcp_server.md)
* Channels
* Day Wrap

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