# More Complicated Benchmarks

Recommended crate:

* [Criterion](https://github.com/bheisler/criterion.rs)

## Setting Up Criterion

In `Cargo.toml`, add:

```toml
[dev-dependencies]
criterion = { version = "0.4", features = [ "html_reports" ] }

[[bench]]
name = "my_benchmark"
harness = false
```

> `[dev-dependencies]` is new! This is a dependency that is *only* loaded by development tools, and isn't integrated into your final program. No space is wasted.

Create `benchmark2/benches/my_benchmark.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

> Taken from the Criterion demo page. The "Optimizing & Debugging Rust" class goes into a lot more detail.

Run `cargo bench` and see the result.

Go to `target/criterion` and you have a full HTML report with statistics.