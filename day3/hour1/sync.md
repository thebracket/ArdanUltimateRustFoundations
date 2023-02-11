# Synchronization Primitives

In the previous example, we wrapped everything in a `Mutex`. Mutex is great: it's a safe wrapper, guarantees single access, and is simple to use. It's not always the type of synchronization you want.

## Understanding Locking

Going back to our Playground sync example:

```rust
use std::sync::Mutex;

struct MyType(usize);

impl MyType {
    const fn new(n: usize) -> Self { // <-- Notice we've added a constant constructor
        Self(n)
    }
}

static SHARED: Mutex<MyType> = Mutex::new(MyType::new(5));

fn main() {
    println!("{}", SHARED.lock().unwrap().0);
}
```

The `SHARED.lock().unwrap()` is two stage:
* `lock()` obtains exclusive access to the interior variable. You can mutate it.
* `unwrap()` catches any errors that may occur.

Let's expand on this a bit:

```rust
fn main() {
    let mut lock = SHARED.lock().unwrap();
    lock.0 += 1;
    println!("{}", lock.0);
}
```

`lock` provides mutable access: you can change the global variable. Nothing else can make changes while you are in there.

## Deadlocks

Now let's make a typo:

```rust
fn main() {
    let mut lock = SHARED.lock().unwrap();
    lock.0 += 1;
    println!("{}", SHARED.lock().unwrap().0);
}
```

Run this, and the Playground helpfully prevents us from spinning forever:

```
/playground/tools/entrypoint.sh: line 11:     8 Killed                  timeout --signal=KILL ${timeout} "$@"
```

We accidentally locked our `Mutex` twice: and Rust didn't give any sort of warning. **Rust provides no protection against deadlocks**. Deadlocks happen at runtime, and most of Rust's safety is compile-time. Rust has no way of knowing the order in which you will wind up running things in a complex application, so it doesn't try.

### Avoiding Deadlocks with `Drop`

To avoid deadlocks, you have to understand how locks work. They use `Drop` to release the lock. So you can fix our problem above by simply dropping `lock` out of scope when you are done with it:

```rust
fn main() {
    {
        let mut lock = SHARED.lock().unwrap();
        lock.0 += 1;
    }
    println!("{}", SHARED.lock().unwrap().0);
}
```

Making *interior scopes* like this is a great, popular way to handle the problem. When `lock` reaches the end of the scope, it is dropped --- releasing the lock. No deadlock.

You can also manually drop a lock:

```rust
fn main() {
    let mut lock = SHARED.lock().unwrap();
    lock.0 += 1;
    std::mem::drop(lock);
    println!("{}", SHARED.lock().unwrap().0);
}
```

This does the same thing, but without introducing a whole new scope. If you need to retain other parts of the scope, that's your escape hatch!

## Types of Lock

There are two popular locking primitives built into Rust. They operate similarly. You've seen `Mutex`, the other popular lock is `RwLock`---the "Read/Write Lock".

`RwLock` works similarly:

```rust
use std::sync::RwLock;

struct MyType(usize);

impl MyType {
    const fn new(n: usize) -> Self { // <-- Notice we've added a constant constructor
        Self(n)
    }
}

static SHARED: RwLock<MyType> = RwLock::new(MyType::new(5));

fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            let read_lock = SHARED.read().unwrap();
            println!("The value of SHARED is {}", read_lock.0)
        });
    }
    std::thread::spawn(|| {
        let mut write_lock = SHARED.write().unwrap();
        write_lock.0 += 1;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));
}
```

This yields:

```
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 6
The value of SHARED is 6
```

The difference is that you can ask for a `read` or `write` lock. You can have an unlimited number of `read` locks active at a time. You can only ever have one `write` lock (and no read locks while it is active).

When `write` activates, it *waits until the read locks are done*. So if you decide to keep a permanent `read` lock somewhere, the writing thread will deadlock.

So if you decide to make your readers sleep:

```rust
fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            let read_lock = SHARED.read().unwrap();
            println!("The value of SHARED is {}", read_lock.0);
            std::thread::sleep(std::time::Duration::from_secs(5));
        });
    }
    std::thread::spawn(|| {
        let mut write_lock = SHARED.write().unwrap();
        write_lock.0 += 1;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));
}
```

Your writer never actually fires in the short time we're running the program:

```
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
```

Dropping the read lock changes this:

```rust
use std::sync::RwLock;

struct MyType(usize);

impl MyType {
    const fn new(n: usize) -> Self { // <-- Notice we've added a constant constructor
        Self(n)
    }
}

static SHARED: RwLock<MyType> = RwLock::new(MyType::new(5));

fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            let read_lock = SHARED.read().unwrap();
            println!("The value of SHARED is {}", read_lock.0);
            std::mem::drop(read_lock);
            std::thread::sleep(std::time::Duration::from_secs(5));
        });
    }
    std::thread::spawn(|| {
        let mut write_lock = SHARED.write().unwrap();
        write_lock.0 += 1;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));
}
```

Giving lots of updates:

```
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 5
The value of SHARED is 6
The value of SHARED is 6
The value of SHARED is 6
The value of SHARED is 6
The value of SHARED is 6
```

## Some Rules of Thumb

* Keep all locks as short as possible.
* If you need to perform a huge update, build the result outside of the lock---and move it in.
