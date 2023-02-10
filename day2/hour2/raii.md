# RAII: Resource Acquisition is Initialization

We've used this a few times already, but I wanted to briefly mention RAII. It's one of the powerful features that helps make Rust a safer language. The feature originated in object-oriented languages such as SmallTalk and C++, and the idiom is important to "Modern" C++. In the OOP world, it's known as a "destructor". Unlike destructors in Java, you can be very sure it will run---but it *isn't* absolutely guaranteed to run if the program panics.

Take the following Playground program:

```rust
struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Destructing")
    }
}

fn main() {
    let x = Droppable;
}
```

If you run it, you'll see "Destructing". Whenever anything that implements the "Drop" trait goes out of scope---or is destroyed with `std::mem::drop`, the `drop` function will run.

This is remarkably handy. Rust uses it for file descriptors (closing the file when `File` leaves scope), locks (automatically relinquish a lock when the lock leaves scope). Network connections automatically close when they leave scope.

You should use it, too. Whenever you acquire a finite resource, make sure you implement `Drop` to guarantee that you relinquish your hold on the program.

Let's take a quick look at [/src/file_lock](/src/file_lock).

```rust
use std::fs::remove_file;
use std::time::Duration;
use std::{path::Path, fs::File};
use std::io::Write;

struct FileLock;

impl FileLock {
    fn new() -> Self {
        let path = Path::new("file.lock");
        if path.exists() {
            panic!("You can't run this program more than once");
        }
        let mut output = File::create(path).unwrap();
        write!(output, "locked").unwrap();

        Self
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let path = Path::new("file.lock");
        remove_file(path).unwrap();
    }
}

fn main() {
    let _lock = FileLock::new();
    // Pretend to do something important
    std::thread::sleep(Duration::from_secs(30));
}
```

When `FileLock` is created, it looks for a file named `file.lock`. If it exists, the program panics. If it doesn't, it makes the file. When `FileLock` is destroyed, the lock is deleted.

Run the program: `file.lock` appears.
Run in a second window: the program panics.

Now hit `ctrl-c` to terminate. The file remains! We don't have time to dive into how to handle this yet, but [This Guide](https://rust-cli.github.io/book/in-depth/signals.html) will give you what you need to handle this eventuality.

As we go into the break, think about how you can combine what you learned about reference counting with a `Drop` handler. You could bind a shared resource, lock access to it, and automatically disconnect when you are done. At very little performance cost.