# Shared Data

You may want to keep the list of prime numbers, rather than just counting it. A naïve approach would be to have a shared list of prime numbers, and add to it.

> We're live-coding. The Github repo is [here](/src/count_primes_shared/)

We replace the counter with a Mutex-wrapped vector:

```rust
static PRIMES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
```

The counting process changes to collecting a vector, and extending the shared vector (with locking):

```rust
threads.push(std::thread::spawn(move || {
    let range = u32::max(2, counter*group) .. (i+1)*group;
    let my_primes: Vec<u32> = range.filter(|n| is_prime(*n)).collect();
    PRIMES.lock().unwrap().extend(my_primes);
}));
```

And summarizing becomes:

```rust
println!("Found {} prime numbers in the range 2..{MAX}", PRIMES.lock().unwrap().len());
```

Pretty straightforward: you've created a shared (static) vector, wrapped it with a Mutex and lock it/add to it when you have results. It's low on collisions, but if one thread has answers at the same time as another---the second one has to wait before it can insert data.

## Per-Thread Results

> We're live-coding. The Github repo is [here](/src/count_primes_shared2/)

Let's remove the `Mutex` requirement, and the shared `primes` vector altogether. Then we'll replace our list of threads with a more complex type:

```rust
let mut threads: Vec<JoinHandle<Vec<u32>>> = Vec::with_capacity(N_THREADS as usize);
```

We're telling Rust that the threads we are tracking will *return* a vector of `u32`.

Now we make that true, by just returning the list of prime numbers instead of appending it to a shared structure:

```rust
for i in 0 .. N_THREADS {
    let counter = i;
    threads.push(std::thread::spawn(move || {
        let range = u32::max(2, counter*group) .. (i+1)*group;
        range.filter(|n| is_prime(*n)).collect()
    }));
}
```

When it comes time to wait for the threads, via the join handles---we retrieve each in turn. `join` returns a `Result` type, so we want to handle potential errors:

```rust
let mut primes = Vec::new();
for thread in threads {
    if let Ok(new_primes) = thread.join() {
        primes.extend(new_primes);
    } else {
        println!("Something went wrong");
    }
}
```

Finally, we can just print the size of the vector with no locks:

```rust
println!("Found {} prime numbers in the range 2..{MAX}", primes.len());
```

Comparing the `Mutex` version (approx 0.283 seconds) with the lock-free version (approx 0.263 seconds)---lock free is faster when you have potential contention, **but its not that much faster**. `Mutex` is really, really fast.