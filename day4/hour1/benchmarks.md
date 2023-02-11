# Simple Benchmarks

If you decide to take the "Debugging & Optimization" class, we'll delve into benchmarks in a lot of detail.

## Quick and Dirty

A quick and dirty way to benchmark operations is to use the built in `Instant` and `Duration` types. For example:

[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=52dedaf0c6963c7deb6a2728425b78c5)

```rust
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut i = 0;
    for j in 0 .. 1_000 {
        i += j*j;
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {} nanos", elapsed.as_nanos());
    println!("{i}");
}
```

This is handy when you just want to get a handle on how long something takes to run. It's not 100% accurate, because reading the clock isn't instantaneous.

## Embedding Benchmarks in your Tests

There are some quite complicated test suites available, including `criterion`---which is excellent. We won't get to that in this class, but we will learn how to make use of unstable Rust features.

Make sure that you have the `nightly` toolchain installed:

```
rustup install nightly
```

> Live-coded. See the Github link [here](/src/bench/)

[Documentation for this unstable feature](https://doc.rust-lang.org/1.4.0/book/benchmark-tests.html)

Now we'll write our test system, using the basic `Cargo` generated empty library as a base:

```
cargo init bench --lib
```

```rust
#![feature(test)]

extern crate test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        b.iter(|| add(2, 4));
    }
}
```

You can run your benchmark with:

```
cargo +nightly bench
```

This does carry a pretty big downside over using a full benchmark suite such as `Criterion`: you just required that your build use Rust's `nightly` channel.

The easiest thing to do is to comment out the benchmark when you aren't using it. 