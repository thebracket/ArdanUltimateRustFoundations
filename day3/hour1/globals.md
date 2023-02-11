# Global Variables

> This will be live-coded in the Rust Playground.

Most languages let you use global variables, and it's not at all uncommon to use them to share data. Global access patterns divide into:

* Actual global variables.
* Static members of types.
* Singletons---structures designed to hold a single copy of some data and share it.

New Rustaceans are often surprised that this doesn't work:

```rust
let shared = 5;

fn main() {
    println!("{shared}");
}
```

The compiler error message is:

```
error: expected item, found keyword `let`
 --> src/main.rs:1:1
  |
1 | let shared = 5;
  | ^^^ consider using `const` or `static` instead of `let` for global variables
```

**Rust does not support simple global variables**. Here's why:

* In many languages, initialization order is an open question. When exactly are non-constant global variables initialized?
* You can never be sure where changes to global variables are coming from. This makes it effectively impossible for the borrow checker to enforce its rules.
* Rust assumes that you are in a multi-threaded environment. That means it has to assume the worst: changes might come from anywhere, at any time.

## Make it a Constant

In this case, `shared` is immutable and could be a constant:

```rust
const SHARED: usize = 5;

fn main() {
    println!("{SHARED}");
}
```

Constants can be global, public global. Rust knows that they can never change. See the [Constants](./constants.md) section.

This is great for variables that constant, but what about actual shared data?

## Static Variables

Let's take the compiler's other piece of advice and make a static variable:

```rust
static SHARED: usize = 5;

fn main() {
    println!("{SHARED}");
}
```

That worked! Unfortunately, `SHARED` is still immutable---you can't change it. It's effectively a constant (and probably turned into one with compiler optimizations). So let's make it mutable:

```rust
static mut SHARED: usize = 5;

fn main() {
    println!("{SHARED}");
}
```

The variable declaration worked, but the compiler really doesn't like you *using* the variable:

```
error[E0133]: use of mutable static is unsafe and requires unsafe function or block
 --> src/main.rs:4:16
  |
4 |     println!("{SHARED}");
  |                ^^^^^^ use of mutable static
  |
  = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

### Abandon all Safety

Ok, so what about going unsafe?

```rust
static mut SHARED: usize = 5;

fn main() {
    unsafe {
        println!("{SHARED}");
    }
}
```

That works. You can even change `SHARED`:

```rust
static mut SHARED: usize = 5;

fn main() {
    unsafe {
        SHARED += 1;
        println!("{SHARED}");
    }
}
```

That also works!

Now for the bad news. Unless you *have no alternative*, **DO NOT DO THIS**. You:

* Turned off Rust's ability to detect data races. You could write the buggy prime number calculator this way.
* You'll have `unsafe` markers *everywhere* you use the data. `unsafe` doesn't actually mean "this isn't safe"---it means you are promising that you know what you are doing, and it's not Rust's fault if something goes awry. You have that power, use it responsibly.

### Safely Share Types with Interior Mutability

Remember back in [lifetimes](../../day2/hour2/lifetimes.md) we talked about interior mutability patterns? This is another case in which you can use interior mutabilty to safely share data. There's one additional caveat: the constructor *must* be a constant function!

Atomic primitives automatically support a constant constructor, so you can use them as safely shared global variables:

```rust
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static SHARED: AtomicUsize = AtomicUsize::new(5);

fn main() {
        println!("{}", SHARED.load(Ordering::Relaxed));
}
```

That's quite a bit more typing, but you are safe from data-races---and have a global variable.

You *can* rig up a nasty-looking combination of `Arc`, `RefCell` and similar to make a global variable work, but it's *much* easier to use a synchronization primitive:

```rust
use std::sync::Mutex;

static SHARED: Mutex<usize> = Mutex::new(5);

fn main() {
        println!("{}", *SHARED.lock().unwrap());
}
```

We'll be diving into synchronization primitives [in a moment](./sync.md). Let's stick to global variables for just a little longer.

## Non-Constant Initializers

Not every type can be initialized in a `const` way. You don't always know your inputs ahead of time. So this won't work:

```rust
use std::sync::Mutex;

struct MyType(usize);

impl MyType {
    fn new(n: usize) -> Self {
        Self(n)
    }
}

static SHARED: Mutex<MyType> = Mutex::new(MyType::new(5));

fn main() {
        println!("{}", SHARED.lock().unwrap().0);
}
```

In this case, you can put change `new` to `const fn new` and it'll work. But the issue here is that you need a constructor that isn't constant.

You could work around it by having a constant initializer, and then remember to call some sort of `update()` function. That would work, but it's risky: what if you forget to `update`? What if you call `update` at the wrong time?

## Lazy Singletons

> We're switching back to Visual Studio Code since this requires a dependency. The Github is [here](/src/globals/)

In `Cargo.toml`, add a dependency to a crate called `once_cell`. Once cell is popular enough that the Rust error messages suggest using it, and there's some discussion about merging it into the core.

Then use the `Lazy` type:

```rust
use std::sync::Mutex;

use once_cell::sync::Lazy;

struct MyType(usize);

impl MyType {
    fn new(n: usize) -> Self {        
        Self(n)
    }
}

static SHARED: Lazy<Mutex<MyType>> = Lazy::new(|| Mutex::new(MyType::new(5)));

fn main() {
    println!("{}", SHARED.lock().unwrap().0);
}
```

This is a very safe construct:
* `Lazy` will run the initialization for your type *the first time it is accessed*. You can't forget to set it up.
* `Mutex` ensures that whatever you are storing inside is protected against data races.
* `Mutex` ensures that the contents are `Send+Sync` (see the next section), so you can have pretty much anything in there.

> Will the `Lazy` type be dropped when you quit?
 
Let's try it:

```rust
use std::sync::Mutex;

use once_cell::sync::Lazy;

struct MyType(usize);

impl MyType {
    fn new(n: usize) -> Self {        
        Self(n)
    }
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Drop");
    }
}

static SHARED: Lazy<Mutex<MyType>> = Lazy::new(|| Mutex::new(MyType::new(5)));

fn main() {
    println!("{}", SHARED.lock().unwrap().0);
}
```

`cargo run` shows us:

```
5
```

**You can't count on destructors running for global types.** It's probably a bad idea to have a global that requires some form of resource locking anyway, but don't count on `Drop` to cleanup after your globals.
