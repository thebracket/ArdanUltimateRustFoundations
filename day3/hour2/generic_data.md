# Generic Data

You've seen how to implement generic functions. Let's take some time to implement our own generic data store. The objective here is to show that everything you've learned about generic functions also applies to data structures---and you will learn to use generics inside member functions.

> This is live-coded. The GitHub version is [here](/src/generic_data/)

We're going to make a sort-of useful data structure: a `StableVec` that retains ID numbers even after entries are removed. This allows you to reuse index numbers, at the expense of wasting memory.

## Creating a Struct with a Generic Type

```rust
#[derive(Debug)]
struct StableVec<T> {
    data: Vec<Option<T>>
}
```

You don't need to derive `Debug`, but it helps. You apply a `<T>` to the type, and can use the `<T>` as a placeholder inside the type.

## Creating a Constructor

The `impl` is a bit different:

```rust
impl <T> StableVec<T> {
```

You *declare* `T` after the implementation, and use it to fulfill the requirements of `StableVec<T>`. Let's add some functions:

```rust
impl <T> StableVec<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn push(&mut self, item: T) -> usize {
        let id = self.data.len();
        self.data.push(Some(item));
        id
    }

    fn remove(&mut self, id: usize) {
        self.data[id] = None;
    }

    fn get(&self, id: usize) -> &Option<T> {
        &self.data[id]
    }
}
```

We can implement `Index` to support access via `[3]`:

```rust
impl<T> Index<usize> for StableVec<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
```

And let's test it:

```rust
fn main() {
    let mut store = StableVec::<String>::new();
    let a = store.push("A".to_string());
    let b = store.push("B".to_string());
    let c = store.push("C".to_string());
    store.remove(b);
    println!("{:?}", store.get(a));
    println!("{:?}", store.get(b));
    println!("{:?}", store.get(c));
    println!("{:?}", store[c]);
}
```