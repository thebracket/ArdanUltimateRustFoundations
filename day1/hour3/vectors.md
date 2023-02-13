# Vectors

In the previous session, we defined an *array* of users:

```rust
pub fn get_users() -> [User; 3] {
    [
        User::new("herbert", "password", LoginAction::Accept(Role::Admin)),
        User::new("bob", "password", LoginAction::Accept(Role::User)),
        User::new("fred", "password", LoginAction::Denied(DeniedReason::PasswordExpired)),
    ]
}
```

It's quite unlikely that the project will only ever have exactly 3 users, and recompiling the whole program everytime you add one is cumbersome. Let's use a `Vec` - or "Vector".

> This section will be live-coded. The GitHub version is [here](/src/auth_vec/)

A *Vec* is just like an array, but:

* Vectors are *resizable*. You can add or remove elements at any time.
* Like arrays, Vectors guarantee that all data will be stored contiguously in memory - making for very fast access on systems with plenty of cache.
* Arrays store their data on the *stack* - limiting size, but almost guaranteeing that they are in the cache. Vectors store all of their data on the *heap* - giving size limited by your RAM, but slowing access slightly with a pointer access. The delay is negligible---nanoseconds---in most cases.

Vectors work a lot like arrays. Replace your `get_users()` function (in `authentication`) as follows:

```rust
pub fn get_users() -> Vec<User> {
    vec![
        User::new("herbert", "password", LoginAction::Accept(Role::Admin)),
        User::new("bob", "password", LoginAction::Accept(Role::User)),
        User::new("fred", "password", LoginAction::Denied(DeniedReason::PasswordExpired)),
    ]
}
```

Notice:
* Instead of an array, we return `Vec<User>`. The angle brackets are *generic type indicators* - indicating that this particular vector holds `User` types.
* The `[..]` has been replaced with `vec![..]`. `vec!` is a macro that creates a vector from an array-like initialization.
* The rest of the program still works. Vectors convert to slices *exactly* like arrays---no other changes are required.

---

> Vectors are one of the most used constructs in the Rust language. They are everywhere. Let's take a couple of minutes to understand how they work.

Let's peek inside the `vec!` macro:

```rust
(<[_]>::into_vec(
    #[rustc_box]
    $crate::boxed::Box::new([
        (User::new("herbert", "password", LoginAction::Accept(Role::Admin))),
        (User::new("bob", "password", LoginAction::Accept(Role::User))),
        (User::new(
            "fred",
            "password",
            LoginAction::Denied(DeniedReason::PasswordExpired),
        )),
    ]),
))
```

This shows you what's actually happening under the hood:

1. Create a `Box`. A box is a pointer to the heap. Boxes are *smart* pointers - they will automatically dispose of their contents when they cease to exist. No memory leaks!
2. Inside the box---in heap memory---your `User` structures are created in an array.
3. `into_vec` transforms the array into a vector. Since the memory is already on the heap, this is more-or-less instantaneous. Rust is a *move by default* language---so the data is moved, not copied.

Let's take a look at how `Vec` is implemented. In Visual Studio Code, you can click a `Vec` and press `F12` to open its definition. The definition code looks like this:

```rust
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```

For such an important type, it's quite straightforward:
* `buf` defines the "raw vector" - a contiguous block of memory, holding the vector's content.
* `len` lists how many items are in the vector.

Let's dive a bit deeper in and look at `RawVec`:

```rust
pub(crate) struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    cap: usize,
    alloc: A,
}
```

Again, a very short type. It's a little more complicated, though:

* `ptr` is a *unique pointer*---a single ownership pointer that deletes itself when it goes out of score---to a block of memory.
* `cap` lists the current *vector capacity*---how many items can the currently allocated pool of memory hold.
* `alloc` is the currently active allocator. This is usually the allocator provided by your Operating System. Changing allocators is sometimes an optimization, but not really a foundation-level topic---so let's not dive in there yet!

So, why do you have two lengths: a *capacity* and a *length*?

Vectors can grow and shrink. If a vector doesn't have available capacity for a new entry, it has to request an all new block of memory from the Operating System, move its contents into the new block, and add the new member to the data store. This is a relatively slow operation, so vectors reserve some extra space ahead of time.

Let's run the following code in the Rust playground:

```rust
fn main() {
    let mut my_vector = Vec::new();
    for i in 0..20 {
        my_vector.push(0);
        println!("Size: {}, Capacity: {}", my_vector.len(), my_vector.capacity());
    }
}
```

> Playground code: [Click Here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cb145c14448cf5854a37f74765c858dd)

Running this code gives you the following output:

```
Size: 1, Capacity: 4
Size: 2, Capacity: 4
Size: 3, Capacity: 4
Size: 4, Capacity: 4
Size: 5, Capacity: 8
Size: 6, Capacity: 8
Size: 7, Capacity: 8
Size: 8, Capacity: 8
Size: 9, Capacity: 16
Size: 10, Capacity: 16
Size: 11, Capacity: 16
Size: 12, Capacity: 16
Size: 13, Capacity: 16
Size: 14, Capacity: 16
Size: 15, Capacity: 16
Size: 16, Capacity: 16
Size: 17, Capacity: 32
Size: 18, Capacity: 32
Size: 19, Capacity: 32
Size: 20, Capacity: 32
```

Let's look at that as a graph (going up to 256 elements):

![](/images/VectorGrowth.png)

**Every time the vector runs out of capacity, it doubles the capacity**.

This is important to think about for a couple of reasons:

* If you know the capacity you need in advance, instead of using `Vec::new` to create an empty vector, use `Vec::with_capacity(n)`. This pre-reserves the amount of data you request.
* If you have a really big vector, adding can be slow if you run out of capacity and it has to reallocate.
* If you are adding to a really big vector, you may wind up using twice as much memory as you intended!
---

## Getting Data into Vectors

> We're back to live-coding.  The GitHub version is [here](/src/auth_vec_exe/)

You've seen the `vec!` macro. There are other ways to add to a vector. The easy one---which we used above---is `push`. Pushing an item adds it to the end of the vector. Open up the `login` program, and let's retrieve the user list - and then manipulate it a bit. This isn't something that normally lives in a login program, but it helps get a feel for working with vectors.

```rust
fn main() {
    let mut users = get_users();
    users.push(User::new("kent", "password", LoginAction::Accept(Role::Limited)));
```

Here, we've changed `users` to be mutable (so we can change it) and used `push` to add another user.

### Deleting with Retain

You can delete vector entries with `retain`. This will delete all users except for "kent". Retain takes a function---*closure*---that returns true if an entry should be kept.

```rust
users.retain(|u| u.username == "kent");
```

### Deleting with Remove

```rust
users.remove(0);
```

This code will remove the vector element at index position 0. Retain is generally a better choice unless you are *sure* that the record is exactly where you think it is.

### Collecting from Iterators

It's really common to iterate over data and collect the results into a vector. Let's quickly collect a list of just the usernames:

```rust
let usernames: Vec<&String> = users
    .iter()
    .map(|u| &u.username)
    .collect();
println!("{usernames:#?}");
```

Notice the `:#?`? That's "pretty print debug information. Since we've trimmed the list down to just "kent", our username list looks like this:

```
[
    "kent",
]
```

Don't sweat the details yet---we'll be doing much more advanced work with data transformations over the next couple of days. This was intended to get you familiar with `Vec`. Many C programmers have complained that when they start using Rust *everything* is a `Vec`---and they are often right, other than `Vec` being awesome!