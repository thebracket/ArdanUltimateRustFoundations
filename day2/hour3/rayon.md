# Using Rayon

`Rayon` is a popular Rust crate that provides easy-to-use solutions for CPU bound problems. Rayon:

* Uses a work-stealing thread pool for tasks.
* Offers `par_iter` to readily transform regular iterator tasks into parallel tasks.
* Is a lot like Intel's `Threaded Building Blocks` library, but with less pain.

> This is live-coded. For the Github, see [here](/src/count_primes_rayon/)

We'll start by making a new project and adding it to our workspace.

```
cargo init count_primes_rayon
```

(Edit `[members]`)

Then we add Rayon to the dependencies:

```
cargo add rayon
```

Now, let's transform our prime counter into an iterator chain:

```rust
fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX:u32 = 200000;
    let now = std::time::Instant::now();

    let count = (2..MAX)
        .filter(|n| is_prime(*n))
        .count();

    let duration = now.elapsed();
    println!("Found {count} primes in {} seconds", duration.as_secs_f32());
}
```

There's no parallelism here. Running it shows that it performs about equally to our single-threaded loop:

```
cargo run --release
Found 17986 primes in 1.0798844 seconds
```

> This is live-coded. For the Github, see [here](/src/count_primes_rayon2/)

Let's use Rayon. In the iterator chain, we need to add one line:

```rust
let count = (2..MAX)
    .into_par_iter()
    .filter(|n| is_prime(*n))
    .count();
```

This will flag errors in the IDE. Let's use it to find the required imports. We end up with:

```rust
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
```

Now when we run the program:

```
cargo run --release
Found 17984 primes in 0.1061489 seconds
```

That was almost too easy. Rayon makes it very easy to build parallel versions of existing iterator calculations.

## Rayon Can Manage Your Threads

