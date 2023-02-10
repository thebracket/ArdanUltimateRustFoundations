# Atomically Counting Primes

So we saw that Rust prevented us from silently corrupting our calculations. I tend to think that's enough reason to use a language for any data I care about. Let's see how it does at resolving the issue.

> Live-coded. Github is [here](/src/count_primes_atomic/)

We change `counter` to be `static`---shared program wide. We change the type to an `AtomicUSize`. Atomic variables are typically implemented by the CPU, and have a very low overhead to use. They also *guaranty* that you won't run into concurrency issues. You can specify an order of operations; in this case, we're going to use the basic `std::sync::atomic::Ordering::Relaxed`. We don't need anything fancier.

```rust
use std::sync::atomic::AtomicUsize;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let t1 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (2 .. MAX/2).filter(|n| is_prime(*n)).count(), 
            std::sync::atomic::Ordering::Relaxed
        );
    });
    let t2 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (MAX/2 .. MAX).filter(|n| is_prime(*n)).count(), 
            std::sync::atomic::Ordering::Relaxed
        );
    });
    t1.join();
    t2.join();
    println!("Found {} prime numbers in the range 2..{MAX}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
 }
```

Run the program `cargo run --release` a few times. Each time you get the same answer:

```
Found 17984 prime numbers in the range 2..200000
```

Let's add a timer to see how using 2 threads instead of one helped:

```rust
use std::sync::atomic::AtomicUsize;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let now = std::time::Instant::now();
    let t1 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (2 .. MAX/2).filter(|n| is_prime(*n)).count(), 
            std::sync::atomic::Ordering::Relaxed
        );
    });
    let t2 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (MAX/2 .. MAX).filter(|n| is_prime(*n)).count(), 
            std::sync::atomic::Ordering::Relaxed
        );
    });
    t1.join();
    t2.join();
    let duration = now.elapsed();
    println!("Found {} prime numbers in the range 2..{MAX}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    println!("Execution took {} seconds", duration.as_secs_f32());
 }
```

The output shows:

```
Execution took 0.7973312 seconds
```

Compared with 1.086 seconds in the first version, we've sped things up. Not quite by a linear amount, but better.

## Using Lots of Threads

> Still live-coding. This version is [here](/src/count_primes_atomic_many/)

Let's transform our code to use lots of threads:

```rust
use std::sync::atomic::AtomicUsize;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    const N_THREADS: u32 = 8;

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    // Hold thread handles
    let mut threads = Vec::with_capacity(N_THREADS as usize);

    // Generate all the numbers we want to check
    let group = MAX / N_THREADS;

    let now = std::time::Instant::now();

    for i in 0 .. N_THREADS {
        let counter = i;
        threads.push(std::thread::spawn(move || {
            let range = u32::max(2, counter*group) .. (i+1)*group;
            COUNTER.fetch_add(
                range.filter(|n| is_prime(*n)).count(),
                std::sync::atomic::Ordering::Relaxed
            );
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
    
    let duration = now.elapsed();
    println!("Found {} prime numbers in the range 2..{MAX}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    println!("Execution took {} seconds", duration.as_secs_f32());
 }
 ```

 * We've made a vector for thread handles.
 * We calculate the group size.
 * We count through `N_THREADS` and spawn a thread, using the counter to calculate a range.
    * Notice that we're using `u32::max` to ensure that we don't count 0..1
* We join all the threads.

`cargo run --release` now shows:

```
Found 17984 prime numbers in the range 2..200000
Execution took 0.272477 seconds
```

That's a good speed-up, at the expense of slightly messy code.