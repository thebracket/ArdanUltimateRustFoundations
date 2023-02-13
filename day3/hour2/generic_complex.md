# Complicated Generic Data

The previous example was deliberately straightforward, to let you see how generic data works. When you start to build more complicated generic data-types, you start to have to mix your knowledge of generic data and generic functions together.

> Live coding. The Github is [here](/src/generic_data_complex/).

Let's make a key-value store that itself stores a vector of results:

```rust
use std::collections::HashMap;

struct HashSetData<KEY, VALUE> {
    data: HashMap<KEY, Vec<VALUE>>
}

impl <KEY, VALUE> HashSetData<KEY, VALUE> {
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }
}
```

This compiles! Your `HashSetData` has a generic `KEY` and a generic `VALUE` type - just like a `HashMap`. Each entry in turn stores its own vector of the data. Your constructor works. Great!

Now let's add a function to add data to the data-structure:

```rust
fn add_reading(&mut self, key: KEY, reading: VALUE) {
    if let Some(entry) = self.data.get_mut(&key) {
        entry.push(reading);
    } else {
        self.data.insert(key, vec![reading]);
    }
}
```

Straightforward enough:
* If an entry exists for a given KEY, append the reading to the key's vector.
* If an entry does NOT exist, insert a new entry with the specified KEY and the new reading as the vector's only occupant.

This won't compile, but the error messages tell you *exactly* why not:

```
error[E0599]: the method `get_mut` exists for struct `HashMap<KEY, Vec<VALUE>>`, but its trait bounds were not satisfied
  --> src\generic_data_complex\src\main.rs:15:40
   |
15 |         if let Some(entry) = self.data.get_mut(key) {
   |                                        ^^^^^^^ method cannot be called on `HashMap<KEY, Vec<VALUE>>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `KEY: Eq`
           `KEY: Hash`

error[E0599]: the method `insert` exists for struct `HashMap<KEY, Vec<VALUE>>`, but its trait bounds were not satisfied
  --> src\generic_data_complex\src\main.rs:18:23
   |
18 |             self.data.insert(key, vec![reading]);
   |                       ^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `KEY: Eq`
           `KEY: Hash`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `generic_data_complex` due to 2 previous errors
```

It's the same error, twice:
* `KEY` must support `Eq`---strong equality.
* `KEY` must support `Hash`---being passed to a hashing function and turned into a hash.

Both of those requirements make sense for a `HashMap`: you have to be able to generate hashes, and compare values.

To add this requirement, it might seem logical to add the requirement to the structure itself:

```rust
struct HashSetData<KEY, VALUE> 
where KEY: Eq + Hash
{
    data: HashMap<KEY, Vec<VALUE>>
}
```

This turns into an error message the the `impl` block also needs the same restrictions:

```rust
impl <KEY, VALUE> HashSetData<KEY, VALUE> 
where KEY: Eq + Hash
{
```

That works! You've successfully restricted your structure to data types that conform to the trait requirements. You can update your `main` function to use the new type:

```rust
fn main() {
    let mut readings = HashSetData::<usize, i32>::new();
    readings.add_reading(1, -2);
    readings.add_reading(1, 3);
    readings.add_reading(1, 5);
    readings.add_reading(2, 1);
}
```

Let's prove that it works by adding `println!("{readings:?})");` to the end. That gives an error, so we add `#[derive(Debug)]` to our type.

Great! We can now show that it worked:
```
HashSetData { data: {1: [-2, 3, 5], 2: [1]} }
```

Let's change the data type to a custom structure.

```rust
struct Data(i32);

fn main() {
    let mut readings = HashSetData::<usize, Data>::new();
    readings.add_reading(1, Data(-2));
    readings.add_reading(1, Data(3));
    readings.add_reading(1, Data(5));
    readings.add_reading(2, Data(1));
    println!("{readings:?}");
}
```

Once again, we're told to annotate `Data` with `Debug`. Suppose we want to make it really obvious that you can only add debuggable types to the structure. We change BOTH `struct` requirements to read:

```rust
where KEY: Eq + Hash, VALUE: Debug
```

That changes the error location to the declaration of `readings`---it's clear up-front that it isn't going to work without `Debug` values.

We can derive `Debug` on `data`, and we're all good:

```
HashSetData { data: {2: [Data(1)], 1: [Data(-2), Data(3), Data(5)]} }
```

Let's go a step further. `Data` is actually coming from a sensor somewhere, and we only care about averaging the results.

We add a skeleton function:

```rust
fn print_results(&self) {
    for (key, value) in self.data.iter() {
        
    }
}
```

Huh - we don't know what `VALUE` is, so we don't know how to average it! We also don't necessarily want to force a sensor to handle readings the same way. So we make a new trait:

```rust
trait Sensor {
    fn reading(&self) -> i32;
}
```

Now we can require that our structure only store things that implement `Sensor`:

```rust
where KEY: Eq + Hash, VALUE: Debug + Sensor
```

Which leaves implementing `Sensor` on `Data`:

```rust
impl Sensor for Data {
    fn reading(&self) -> i32 {
        self.0
    }
}
```

Now we can implement our function:

```rust
fn print_results(&self) {
    for (key, value) in self.data.iter() {
        let sum: i32 = value.iter().map(|r| r.reading()).sum();
        let avg = sum / value.len() as i32;
        println!("{key} : {avg}");
    }
}
```

And - oh no, it fails to compile because we aren't guaranteeing that we *can* print out the key. Rust is very picky: the constraints *have* to make sense for everything you do. Fortunately, the compiler error tells you exactly what we need to do:

```
consider further restricting this bound: ` + std::fmt::Display`
```

So our final structure `where` clause looks like this:

```rust
#[derive(Debug)]
struct HashSetData<KEY, VALUE> 
where KEY: Eq + Hash + std::fmt::Display, VALUE: Debug + Sensor
{
    data: HashMap<KEY, Vec<VALUE>>
}

impl <KEY, VALUE> HashSetData<KEY, VALUE> 
where KEY: Eq + Hash + std::fmt::Display, VALUE: Debug + Sensor
{
```

Rust makes it easy to add generic types, but unlike many other languages all of the deduction happens at *compile time*. You need to carefully consider your traits, and build something that works.

We update our `main` function to use the new capability:

```rust
fn main() {
    let mut readings = HashSetData::<usize, Data>::new();
    readings.add_reading(1, Data(-2));
    readings.add_reading(1, Data(3));
    readings.add_reading(1, Data(5));
    readings.add_reading(2, Data(1));
    readings.print_results();
}
```

And run it:

```
2 : 1
1 : 2
```