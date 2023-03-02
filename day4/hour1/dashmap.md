# Avoiding Locks with DashMap

Sometimes, locks really bog you down. Not in speed: in the complexity of your application. You have updates coming in from multiple sources, readers obtaining information, and the locking soup is annoying.

Good news: there are "lock free" structures available in Rust. They typically either emulate a generational memory scheme (similar to Java), or use atomics *internally* and quickly replace old data with new.

## Add Dashmap

Add Dashmap to your project with `cargo add dashmap`. Note that we renamed the `dashmap` project because you can't have a dependency with the same name as your project.

You also need `once_cell` again, `cargo add once_cell`.

> We're livecoding. See [the Github repo](/src/dashmap/) for details.

```rust
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{thread, time::Duration};

static MAP: Lazy<DashMap<usize, usize>> = Lazy::new(DashMap::new);

fn main() {
    let mut threads = Vec::new();

    // Adder Threads
    for i in 0..10 {
        threads.push(thread::spawn(move || {
            for _ in 0..100 {
                if let Some(mut count) = MAP.get_mut(&i) {
                    *count += 1;
                } else {
                    MAP.insert(i, 1);
                }
                std::thread::sleep(Duration::from_secs_f32(0.1));
            }
        }));
    }

    // Reader Threads
    for i in 0..10 {
        threads.push(thread::spawn(move || {
            for _ in 0..20 {
                if let Some(count) = MAP.get(&i) {
                    println!("Count of {i}: {}", *count);
                    std::thread::sleep(Duration::from_secs_f32(0.5));
                }
            }
        }));
    }

    for t in threads {
        let _ = t.join();
    }
}
```

The output shows that despite there not being any locks, we're ticking upwards.

**Going lock-free is not free. Lock-less structures are a little slower.**