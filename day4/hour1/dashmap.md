# Avoiding Locks with DashMap

Sometimes, locks really bog you down. Not in speed: in the complexity of your application. You have updates coming in from multiple sources, readers obtaining information, and the locking soup is annoying.

Good news: there are "lock free" structures available in Rust. They typically either emulate a generational memory scheme (similar to Java), or use atomics *internally* and quickly replace old data with new.

## Add Dashmap

Add Dashmap to your project with `cargo add dashmap`. Note that we renamed the `dashmap` project because you can't have a dependency with the same name as your project.

You also need `once_cell` again, `cargo add once_cell`.

