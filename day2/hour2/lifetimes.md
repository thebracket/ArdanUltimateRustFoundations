# Lifetimes

We're looking at lifetimes and the borrow checker together, because they are inextricably linked. The borrow checker prevents use-after-move, enforces ownership, and ensures that you don't have even the faintest possibility of changing the same piece of data from two places at a time.

Lifetimes attack another very common bug: using a pointer after the item being pointed to is no longer available (it moved, was freed, etc.).

If you're coming from C or C++, lifetimes can be very annoying---but you've probably run into cases where you kept a reference to something and it ceased to exist.

If you're coming from a garbage collected language (like Go, Java, etc.)---or use `shared_ptr` everywhere in C++---you're probably wondering what the big deal is.

Rust is a low-level, systems language without a garbage collector. In order to guaranty that you aren't running into a lifetime issue, Rust imposes some rules---and those rules can take a bit of getting used to.

## Lifetimes with Functions

Take the following useless function:

```rust
fn do_something(x: &i32) {
    println!("{x}"); 
}

fn main() {
    let x = 12;
    do_something(&x);
}
```

You are passing in a *reference* to an `i32`---meaning that `main` is borrowing `x`. When Rust was new, you'd have had to write:

```rust
fn do_something<'a>(x: &'a i32) {
    println!("{x}"); 
}
```

You're declaring that `do_something` is connected to a *lifetime* (denoted by `'`) named `a`. The *borrow* is annotated (as `&'a`) to indicate that it uses the lifetime for that particular borrow. With that knowledge, Rust can be *certain* that whatever was passed in will *live long enough* for `do_something` to run.

The funny thing is, the first version is translated by Rust to include the lifetime---but because it's an easily resolved case, the compiler can do it for you.

> Don't complicate your code with lifetimes when you don't need them!

### Functions that return a reference - and take more than one reference

Where you can't avoid lifetime annotations is any function that returns a reference, and *receives* more than one reference. For example:

```rust
fn get_x(x: &i32, _y: &i32) -> &i32 {
    x
}

fn main() {
    let a = 1;
    let b = 2;
    let _ = get_x(&a, &b);
}
```

This will fail to compile with:
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:32
  |
1 | fn get_x(x: &i32, _y: &i32) -> &i32 {
  |             ----      ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `_y`
help: consider introducing a named lifetime parameter
  |
1 | fn get_x<'a>(x: &'a i32, _y: &'a i32) -> &'a i32 {
  |         ++++     ++           ++          ++

For more information about this error, try `rustc --explain E0106`.
```

Rust can't determine *which* reference lifetime it needs to worry about. In such a simple case, it's obvious---but not to the compiler. You can correct it by annotating lifetimes:

```rust
fn get_x<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 {
    x
}

fn main() {
    let a = 1;
    let b = 2;
    let _ = get_x(&a, &b);
}
```

That's ugly---but it gets the job done.

## Lifetimes for Structures

It's very common to keep a reference around for use later. Rust has no problems with letting you store a reference in a structure, so long as you provide a lifetime annotation:

```rust
struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a Cat
}

fn main() {
    let cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];
    
    let mut feeders = Vec::new();
    for cat in cats.iter() {
        feeders.push(CatFeeder{ cat })
    }
}
```

Now let's try and be a bit object-oriented. The syntax starts to get a bit messy, but the latest edition of Rust allows the borrow-checker and lifetime checker to handle in-vector mutable borrowing and keep a mutable reference around:

```rust
struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a mut Cat
}

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];
    
    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder{ cat })
    }
    
    feeders.iter_mut().for_each(|f| f.feed());
}
```

So far, so good. You had to ugly up `impl<'a> CatFeeder<'a>` to tell it to *implement for lifetime 'a, CatFeeder that requires lifetime 'a*. Verbose, but functional.

Now let's see what the lifetime protection---which sounds like insurance---is doing. We'll take `cats` out of scope before we feed them:

```rust
fn main() {
    let mut feeders = Vec::new();
    {
        let mut cats = vec![
            Cat("Frodo".to_string()),
            Cat("Bilbo".to_string()),
            Cat("Pippin".to_string()),
        ];
        
        for cat in cats.iter_mut() {
            feeders.push(CatFeeder{ cat })
        }
    }
    
    feeders.iter_mut().for_each(|f| f.feed());
}
```

When the scope containing `cats` ends, the `cats` list ceases to exist. In C++, you'll probably get a warning from modern compilers---but the code is valid. It's undefined behavior what will happen when you feed the cats---their pointers are no longer valid. When you compile it in Rust, it shows you exactly what's wrong:

```
error[E0597]: `cats` does not live long enough
  --> src/main.rs:28:20
   |
28 |         for cat in cats.iter_mut() {
   |                    ^^^^^^^^^^^^^^^ borrowed value does not live long enough
...
31 |     }
   |     - `cats` dropped here while still borrowed
32 |     
33 |     feeders.iter_mut().for_each(|f| f.feed());
   |     ------------------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
```

The lifetime checker just saved you from a bug. It's obvious in this context, now imagine that you have a large programming passing and storing pointers all over. Rust may have just saved you from another CVE report.

The real lesson is: keep pointers for *exactly* as long as you need them, and no longer. It's far better to maintain a unique ID for each animal, a central animal collection, and have feeders refer to the ID. That way, you can check that the cat still exists---and if Schrödinger's cat has ceased to exist, you don't try to feed it. 

## Rust does have Garbage Collection, After All

Sometimes, you *need* to pass and keep references. Garbage collected languages like Java or Go make this easier. C++ even makes it possible with `shared_ptr`. Rust has opt-in *reference counting*---a pointer that counts how many times it is being held, and deletes it contents only when nobody is looking at it.

Let's start with the easier-case of only needing read-only, safe, garbage collected pointers.

We can construct a set of reference counted cats, and give their owners a reference to the cat:

```rust
use std::rc::Rc;

struct Cat(String);

struct CatOwner {
    cat: Rc<Cat>
}


fn main() {
    let mut cats = vec![
        Rc::new(Cat("Frodo".to_string())),
        Rc::new(Cat("Bilbo".to_string())),
        Rc::new(Cat("Pippin".to_string())),
    ];
    
    let mut owners = Vec::new();
    for cat in cats {
        owners.push(CatOwner{ cat: cat.clone() });
    }
    
    for owner in owners {
        println!("{}", owner.cat.0)
    }
}
```

This prints out the list of cats. Notice how we're cloning each cat? We're actually cloning the `Rc` - which increments the ownership counter. `Rc` is designed to be cloned, and it's very fast.

Now we rearrange again, to see if we've defeated the lifetime system:

```rust
use std::rc::Rc;

struct Cat(String);

struct CatOwner {
    cat: Rc<Cat>
}


fn main() {
    let mut owners = Vec::new();
    {
        let mut cats = vec![
            Rc::new(Cat("Frodo".to_string())),
            Rc::new(Cat("Bilbo".to_string())),
            Rc::new(Cat("Pippin".to_string())),
        ];
        
        for cat in cats {
            owners.push(CatOwner{ cat: cat.clone() });
        }
    }
    
    for owner in owners {
        println!("{}", owner.cat.0)
    }
}
```

We have! You can use `Rc` when you need *read-only* references to a type, and ensure that the pointer lives long enough to remain valid.

Let's add a `Drop` handler to show when the cats cease to exist (and prove that they are):

```rust
use std::rc::Rc;

struct Cat(String);

struct CatOwner {
    cat: Rc<Cat>
}

impl Drop for Cat {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn main() {
    {
        let mut owners = Vec::new();
        {
            let mut cats = vec![
                Rc::new(Cat("Frodo".to_string())),
                Rc::new(Cat("Bilbo".to_string())),
                Rc::new(Cat("Pippin".to_string())),
            ];
            
            for cat in cats {
                owners.push(CatOwner{ cat: cat.clone() });
            }
        }
        
        for owner in owners {
            println!("{}", owner.cat.0)
        }
    }
    println!("Program end");
}
```

This yields:
```
Frodo
Frodo was dropped
Bilbo
Bilbo was dropped
Pippin
Pippin was dropped
Program end
```

WAIT A SECOND - what happened here? Each cat is dropped after being read. We've fallen afoul of the "move" rule - we've moved each owner out of the list as we iterated it.

Changing `for owner in owners {` to `for owner in owners.iter() {` fixes the issue:

```
Frodo
Bilbo
Pippin
Frodo was dropped
Bilbo was dropped
Pippin was dropped
Program end
```

## When You Absolutely Can't Avoid Storing Mutable Pointers

It's generally worth trying to find an architecture that doesn't keep mutable pointers around for very long, but sometimes it's hard to avoid. 

Let's look at a working example:

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Cat {
    name: RefCell<String>
}

impl Cat {
    fn new(name: &str) -> Self {
        Self {
            name: RefCell::new(name.to_string())
        }
    }
}

struct Owner {
    cat: Rc<Cat>
}

impl Owner {
    fn feed(&self) {
        let mut name_borrow = self.cat.name.borrow_mut();
        *name_borrow += " (purring)";
    }
}

fn main() {
    let cats = vec![
        Rc::new(Cat::new("Frodo")),
        Rc::new(Cat::new("Bilbo")),
        Rc::new(Cat::new("Pippin")),
    ];
    
    let mut owners = Vec::new();
    for cat in cats.iter() {
        owners.push(Owner{ cat: cat.clone() });
    }
    
    for owner in owners.iter() {
        owner.feed();
    }
    
    for owner in owners.iter() {
        println!("{}", owner.cat.name.borrow());
    }
}
```

The pattern we're using here is called *interior mutability*. `Cat` appears immutable, and accessors remain immutable (using `self` rather than `mut self`). `Cat` wraps its parts that can change in a synchronization mechanism, so a cat can change. `RefCell` implements some locking to ensure that `borrow_mut` and `borrow` won't break the memory safety guarantees.

## The Cost of Reference Counting

Using reference counting isn't free, but it is fast. Open [/src/rc_bench/](/src/rc_bench/) and let's compare four methods of storing cats and feeding them.

1. A simple vector of cats, iterated directly. Always going to be the fastest, not amenable to sharing.
2. Reference counted cats with an interior-mutable field.
3. *Atomic* Reference Counted cats with an interior-mutable field. `Arc` is thread-safe.
4. A central `HashMap` of cats, with each cat assigned an ID number.

Running in *debug* mode (`cargo run`), I get the following result:

```
Vector of Cats                827225 μsecs        82 nanos per cat    
RC Cats                       992624 μsecs        99 nanos per cat    
ARC Cats                      1058448 μsecs       105 nanos per cat   
Cat Store                     4561224 μsecs       456 nanos per cat
```

That's really fast. 82 nanoseconds per cat for direct access, but only 105 nanos per cat if you need full thread-safe reference counting. The central cat store is much slower at 456 nanos per cat. That's because it as more work to do.

Let's give it a go in *release* mode, which turns off debug features, enables all optimizations and turns off a few run-time safety checks. `cargo run --release`:

```
Vector of Cats                256167 μsecs        25 nanos per cat    
RC Cats                       340604 μsecs        34 nanos per cat    
ARC Cats                      339002 μsecs        33 nanos per cat    
Cat Store                     2685409 μsecs       268 nanos per cat
```

So there's only an 8 nanosecond penalty for atomic reference counting yours kitties. Once again, the central store is slower---but 268 nanoseconds is still *very* fast.

