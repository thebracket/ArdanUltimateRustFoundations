# Async and Tokio

For CPU-bound problems, Rayon is great. Not every problem is CPU bound---and Rust can *also* do async/await, or "green threading". This model is best for when you are waiting on something---it could be waiting for a file to load, external services to give result, network traffic. You can even mix-and-match the two, but some caution is needed when you do.

"Green" threads aren't the same as threads:

| **Thread**                                | **Green-Thread**                       |
|-------------------------------------------|----------------------------------------|
| Managed by the OS scheduler               | Managed by your `async` runtime        |
| Expensive to create                       | Cheap to crate                         |
| Keep running while other threads are busy | Depend on the runtime to remain active |
| Always separated                          | May or may not be in a thread          |

> We're live-coding. See [here]() for the Github version.

Probably the most popular async runtime is `Tokio`, which can run in single-threaded or parallel mode.

Let's create a new project:

```
cargo init hello_tokio
cd hello_tokio
cargo add tokio -F full
cargo add anyhow
```

We're adding the feature "full"---to include everything from Tokio. A lot of the time you don't need this, but it's handy.

Let's open `main.rs` and make a simple "Hello, Tokio" program:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}
```

So what's new here?
* We're decorating `main` with `#[tokio:main]`. That's a macro that wraps the whole `main` function in another function that initializes the Tokio async system.
* We're returning a `Result`. You can use that in `main` to have errors propagate all the way out when the program crashes.
* `main` is now marked as `async`. That doesn't do a lot here, but you can't call `async` functions from a regular function without having access to an async runtime to call.

There's nothing async here. Give it a run. It has a lot to compile---Tokio with all the bells and whistles does a lot.

Let's make our first `async` function:

```rust
async fn hello(n: u32) {
    println!("Hello {n}");
}
```

We'll try to call it like a normal function:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hello(1);
    Ok(())
}
```

It didn't do anything! In fact, Clippy even shows a warning. `async` is actually a decorator---when you run an `async` function, it returns a `Future`, containing your program. It hasn't *run* the function, it's given you a handle with which you might choose to run the function later.

The easiest way to run a `Future` is to `await` it:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hello(1).await;
    Ok(())
}
```

When `hello` runs, it returns a `Future`. `await` means "run the future, and wait until it finishes." This prints `hello(1)`.

You can use the `join!` macro from Tokio to spawn multiple futures at once, and wait for all of them:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    join!(
        hello(1), hello(2), hello(3), hello(4)
    );
    Ok(())
}
```

And async functions can call (and await, join, etc.) other async functions:

```rust
async fn hello(n: u32) {
    println!("Hello {n}");
    if n < 10 {
        hello_child(n*10).await
    }
}

async fn hello_child(n: u32) {
    println!("Hello again {n}");
}
```

> Note that attempting to recurse in `async` land becomes painful very quickly. We're not going there right now!

What if you want to fire off a task, and not wait for it?

```rust
async fn hello(n: u32) {
    println!("Hello {n}");
    if n < 10 {
        spawn(hello_child(n*10));
    }
}
```

You *can* `await` on spawn if you want---otherwise it fires off on its own as soon as it leaves the active scope.

## Blocking

This looks a lot like threads, but with a different interface. It isn't! Let's mess things up.

```rust
async fn hello_child(n: u32) {
    println!("Hello again {n}");
    std::thread::sleep(Duration::from_secs(1));
}
```

Despite being joined, the whole program stalls at the sleeping thread. Tokio has a thread-pool, but by default it's *cooperatively* managing green-thread scheduling. When you tell the thread to sleep, you are pausing the entire Tokio runtime---or the currently allocated thread, it's not easy to find out which.

Because of this, Tokio has re-implemented large parts of the standard library. Here's a Tokio-safe version:

```rust
async fn hello_child(n: u32) {
    println!("Hello again {n}");
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

Now `hello_child` yields control back to Tokio, allowing the other threads to continue. All of the `sleep` commands finish together.

Sometimes, you really do need to block and wait for something to finish. You do that by calling `spawn_blocking`:

```rust
let _ = spawn_blocking(|| std::thread::sleep(Duration::from_secs(1))).await;
```

This *will* activate a thread in the thread-pool (if there is one) amd wait for it to return. The `await` lets Tokio know that the green-thread itself is paused; other green-threads can keep running, and when the blocking task returns the green-thread resumes.

That's a quick overview of using async with Tokio. The rules to remember are:

* You **can't** call an `async` function directly from a normal function.
* You **can** call a regular function from an `async` function.
* Futures aren't threads. If you have to block, let the runtime know - or you pause the world.
* When you are in a blocking context, you can do whatever you want. You can even use `rayon`! Getting back *into* the async context is tricky, try to think in terms of independent operations that return results.
* Recursion in green-thread land is hard.
