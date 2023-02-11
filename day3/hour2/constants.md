# Constants

This is one area that Modern C++ is ahead of Rust. C++ lets you do some amazing compile-time calculations, and is very flexible about making functions `constexpr`. The downside being that `constexpr` doesn't *guaranty* that a function is constant---it's more like a suggestion.

## Constant Variables

You've used these already:

```rust
const MY_CONST: usize = 12;
```

They are great, and work just like you'd expect. Set a value, and it works everywhere.

You can do calculations in constants:

```rust
const A: usize = 5;
const B: usize = 6;
const C: usize = A * B;
```

These are evaluated at compile time. If `A` and `B` are never used, other than as parts of the calculation---they will be eliminated from optimized builds.

## Compile-Dependent Constants

You can use compile-time declarations to adjust constants depending upon compilation environment:

```rust
#[cfg(debug)]
const A: usize = 3;

#[cfg(release)]
const A: usize = 4;
```

A `debug` build will have a different value to a `release` build. You might gate by the type of operating system on which the program is compiled:

```rust
#[cfg(target_family = "unix")]
const A: usize = 3;
#[cfg(not(target_family = "unix"))]
const A: usize = 4;

fn main() {
    println!("{A}");
}
```

(The Rust Playground is running a Unix!)

## Constant Functions

You can add the `const` keyword to functions to have the function execute at compile time:

```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const A: usize = add(4, 6);

fn main() {
    println!("{A}");
}
```

You can even use the constant function with dynamic inputs:

```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const A: usize = add(4, 6);

fn main() {
    let mut i = 5;
    i += 3;
    println!("{}", add(A, i));
    println!("{A}");
}
```

So let's try and make a more complicated constant function:

```rust
const fn loopy() -> usize {
    let mut n = 1;
    for i in 0..20 {
        n += i;
    }
    n
}

const A: usize = loopy();

fn main() {
    println!("{A}");
}
```

We've added a *mutable* variable inside a constant. Surely that will fail? No! The error is that you aren't allowed to use `for` loops in constant functions. This is because the `Iterator` type is inherently not constant.

Transforming to a `while` loop works fine:

```rust
const fn loopy() -> usize {
    let mut n = 1;
    let mut i = 0;
    while i < 20 {
        n += i;
        i += 1;
    }
    n
}
```

You can offload a fair amount of work this way: calculating constants up-front means one less calculation to do at run-time.

## What You Can't Do at Compile Time

Unfortunately, there's a pretty big list of things you can't do at compile time --- yet. The Rust Core team are improving this.

* You can't use floating point numbers, except as direct constants. `const n: f32 = 1.0` is fine. A function using floating point numbers won't work.
* You can't use iterators.
* You can't connect to external data sources - OTHER than files. The macros `include_str!` and `include_bytes!` can embed files in your binary.
