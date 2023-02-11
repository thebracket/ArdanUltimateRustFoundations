# Traits

You've already learned the basic building blocks that make up Rust: most of the standard library is some combination of structures, vectors and safety features. The missing piece is the `Trait`. We've *used* a lot of traits, and *implemented* quite a few of them.

Let's make our own trait.

In the playground, let's make a minimal program with a trait:

```rust
trait Animal {
    fn make_noise(&self);
}

fn main() {
    
}
```

This doesn't *do* anything. You can't make an `Animal`. `let cat = Animal{};` fails with a message that it `expected struct, variant or union type, found trait "Animal"`.

Traits are like *interfaces* in other languages. They describe a promise: if a type implements this trait, it will implement functions contained within this trait.

Here's a `Cat` type that implements the `Animal` trait:

```rust
trait Animal {
    fn make_noise(&self);
}

struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
}

fn main() {
    let cat = Cat{};
    cat.make_noise();
}
```

As you'd expect, it outputs `Meow`.

## Trait defaults

You can make a trait implement functions by default. If a type implements the function, it is overridden---otherwise it runs as default.

Here's a default implementation:

```rust
trait Animal {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}
```

And here's a defaulted animal:

```rust
struct Tortoise;

impl Animal for Tortoise {}
```

And calling both:

```rust
fn main() {
    let cat = Cat{};
    cat.make_noise();
    let tortoise = Tortoise{};
    tortoise.make_noise();
}
```

Gives the expected output:

```
Meow
Who knows what noise I make?
```

> It's worth pointing out that Rust is NOT an object-oriented language. You can use traits in a similar fashion, but you can not implement inheritance and inheritance trees.

## Data in traits

Based on what you've seen, it might be tempting to think that you can do this:

```rust
trait Animal {
    name: String,

    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}
```

You can't. Traits don't contain any data, they just contain functions.

You *can* reference the internals of a type from a trait implementation:

```rust
struct Cat {
    noise: String
}

impl Animal for Cat {
    fn make_noise(&self) {
        println!("{}", self.noise);
    }
}

fn main() {
    let cat = Cat{ noise: "meow".to_string() };
    cat.make_noise();
}
```

The `Cat` implementation of `Animal` knows for sure that you are working with a cat, so `&self` will bring up a reference to a `Cat` type.

But this won't compile:

```rust
trait Animal {
    fn make_noise(&self) {
        println!("{}", self.noise)
    }
}
```

You can't *guaranty* that the trait has a `noise` field---and since you can't put fields in traits, that's not going to work.

## Trait Dependencies & Generic Functions

Going back to:

```rust
trait Animal {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}

struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
}

struct Tortoise;

impl Animal for Tortoise {}

fn main() {
    let cat = Cat{};
    cat.make_noise();
    let tortoise = Tortoise{};
    tortoise.make_noise();
}
```

Let's make a generic function that requires our trait:

```rust
fn pet<A: Animal>(animal: A) 
{
    animal.make_noise()
}

fn main() {
    let cat = Cat{};
    let tortoise = Tortoise{};
    pet(cat);
    pet(tortoise);
}
```

Now let's make a second trait:

```rust
trait Tame {}
```

Let's pretend for a minute that cats are really tame:

```rust
impl Tame for Cat {}
```

You don't want to pet a non-tame animal, so let's adjust our function:

```rust
fn pet<A: Animal + Tame>(animal: A) 
{
    animal.make_noise()
}
```

You can add traits together to require that a generic variable has both traits. If you compile now, you get the error:

```
error[E0277]: the trait bound `Tortoise: Tame` is not satisfied
```

## Polymorphic Traits

Let's once again revert to the simple case.

```rust
trait Animal {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}

struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
}

struct Tortoise;

impl Animal for Tortoise {}

fn main() {
    let cat = Cat{};
    cat.make_noise();
    let tortoise = Tortoise{};
    tortoise.make_noise();
}
```

You're probably wondering how to store variables that all implement a trait, but aren't of the same type, into a collection.

Let's try the obvious approach:

```rust
fn main() {
    let cat = Cat{};
    let tortoise = Tortoise{};
    
    let animals = vec![cat, tortoise];
}
```

That's not going to work, because `Vec` derives its type from the first type it sees--which is a `Cat`. Since tortoises aren't cats, they aren't going to sit together in a simple vector.

You probably remember from other languages that you typically make polymorphic objects with `new`. Rust doesn't have a `new`, but `Box` is the equivalent: a smart pointer. Again, the simplest option doesn't work:

```rust
let mut animals = Vec::new();
animals.push(Box::new(cat));
animals.push(Box::new(tortoise));
```

`Vec` is again deriving its type as `Box<Cat>`---because that's the first type it sees. So let's try giving the `Vec` a type upfront:

```rust
let mut animals: Vec<Box<Animal>> = Vec::new();
```

The error message tells you exactly what to do: `error[E0782]: trait objects must include the "dyn" keyword`. The Rust devs slipped up and called them objects, too!

Instead, you have to tell Rust to explicitly turn on *dynamic dispatch*:

```rust
fn main() {
    let cat = Cat{};
    let tortoise = Tortoise{};
    
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(cat));
    animals.push(Box::new(tortoise));
    for animal in animals.iter() {
        animal.make_noise();
    }
}
```

That worked! You could keep defining animals all day if you want.

**Theory time**: Why did that work, and why was `dyn` needed?

By default, Rust operates on concrete types with static dispatch. It knows at compile time what it must do, and is able to do it very quickly.

The `dyn` keyword means "the actual type may change" --- its dynamic. That implies some plumbing under the hood. Objects in OOP languages maintain a `vtable`. A "virtual dispatch table". Every type that *might* appear in the loop needs an entry in the `vtable`---mapping to the function to call.

So calling the function now requires an extra step: Rust checks the type at *runtime*, finds the pointer to the implementation of `make_noise` that applies in this case, and runs that function. It's not a big overhead, but it is slowing things down. Fortunately, years of C++ development has made LLVM very smart and fast about optimizing away the `vtable` whenever it can.

## Making Traits Require Other Traits

So you decide that every animal must support `Debug`. You can accomplish this as follows:

```rust
trait Animal: std::fmt::Debug {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}
```

We're using the long name because `Debug` *also* refers to a derive option. This works - compiling shows that:

```
error[E0277]: `Cat` doesn't implement `Debug`
error[E0277]: `Tortoise` doesn't implement `Debug`
```

Let's add Debug:

```rust
#[derive(Debug)]
struct Cat;

#[derive(Debug)]
struct Tortoise;
```

Now we can debug-print the animal in the loop:

```rust
for animal in animals.iter() {
    println!("{:?}", animal);
    animal.make_noise();
}
```

This is very powerful, because now you are requiring that anything that implements your interface *must* support other interfaces, as well as your own. Let's add a second requirement:

```rust
trait Animal: std::fmt::Debug + std::fmt::Display {
```

So now the implementing types *have* to be printable. You can implement display as follows:

```rust
impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cat")
    }
}

impl std::fmt::Display for Tortoise {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tortoise")
    }
}
```

Now you can just print the animal like a primitive:

```rust
for animal in animals.iter() {
    println!("{animal}");
    animal.make_noise();
}
```

## What if you REALLY need to know the concrete type?

I emphasize *really*, because this is messy.

Rust provides a type called `Any` that applies to anything. Rust really isn't into inheritance and polymorphism, so don't expect much help.

At the top, import:
```rust
use std::any::Any;
```

Then change your `Animal` trait to require `Any`, and *also* require that recipients implement a function named `as_any`:

```rust
trait Animal: std::fmt::Debug + std::fmt::Display + Any {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
    
    fn as_any(&self) -> &dyn Any;
}
```

Then you have to implement `as_any` for every single implementing type:

```rust
impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Animal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

It's the same every time. Finally, in your main loop you can do a runtime check to see if you've received a `Cat`---and "downcast" to use the Cat's native structure type:

```rust
for animal in animals.iter() {
    if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
        println!("We have access to the cat");
    }
    println!("{animal}");
    animal.make_noise();
}
```

Despite being painful, this can be useful. If you have a polymorphic list, and need to access native behavior for a type---this is how you do it. There are crates that help.

[Here's a link to the full solution on the Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a5ca5b7768ea8c13c5d04a137a30bcad)
