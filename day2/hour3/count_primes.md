# Counting Prime Numbers - Badly

As we start to look at concurrency, we need a way to keep the CPU really busy. There are lots of good ways to determine if a number is prime. This isn't one of them:

```rust
fn is_prime(n: u32) -> bool {
   (2 ..= n/2).all(|i| n % i != 0 )
}
```

> This is live-coded. The GitHub repo is [here](/src/count_primes/)

Let's quickly unit test our function to make sure that it works. I grabbed a list of prime numbers from: [https://en.wikipedia.org/wiki/Prime_number](https://en.wikipedia.org/wiki/Prime_number)

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_hundred_primes() {
        // List obtained from: https://en.wikipedia.org/wiki/Prime_number
        let primes: Vec<u32> = (2..100).filter(|n| is_prime(*n)).collect();
        assert_eq!(
          primes,
           [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
        );
     }
}
```

And we'll do a very simple loop with a counter to count prime numbers:

```rust
fn main() {
    let mut count = 0;
    let now = std::time::Instant::now();
    for i in 2 .. MAX {
        if is_prime(i) {
            count += 1;
        }
    }
    let time = now.elapsed();
    println!("Found {count} primes in {} seconds", time.as_secs_f32());
}
```

The result in release mode is:

```
Found 17984 primes in 1.0891765 seconds
```

The objective here is to heat the room up with our CPU. So we aren't too worried that it took 6 seconds in debug mode, and 1.08 seconds in release mode on my PC.

We've built a really inefficient prime number finder. The reason for doing this is that we're going to dive into concurrency, and make this CPU-bound problem faster.
