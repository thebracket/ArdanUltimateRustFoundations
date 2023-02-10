# NewTypes

Aside from memory safety, fearless concurrency, no NULLs and explicit handling of errors---much of Rust's safety comes from being a very strongly typed language.

Despite this, it's not at all uncommon to run into problems.

[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c550903a40a0e5855aa0117fbc3e9ed6)

```rust
fn project_angle(start: (f32, f32), angle: f32, radius: f32) -> (f32, f32) {
    (
        0.0 - (start.0 + radius * f32::sin(angle)),
        start.1 + radius * f32::cos(angle),
    )
}

fn main() {
    let start = (0.0, 0.0);
    let finish = project_angle(start, 180.0, 10.0);
    println!("{finish:?}");
}
```

Ignoring the lack of documentation, this is a pretty common function. It takes a starting point, projects a point at `angle` degrees, over `distance`. It returns the final coordinates.

The result:

```
(8.011526, -5.984601)
```

**There's a *lot* wrong with this function.**

For one thing, if you're projecting by 180 degrees---you'd expect either X or Y to remain 0. Oops - the function requires *radians*. It doesn't say that anywhere, and it doesn't protect itself against bizarre data. So let's just feed it some good data:

```rust
let finish = project_angle(start, 180.0f32.to_radians(), 10.0);
```

That gives a more reasonable response:

```
(8.742278e-7, -10.0)
```

We *could* make things more clear by changing some parameter names:

```rust
fn project_angle(start: (f32, f32), angle_radians: f32, radius: f32) -> (f32, f32) {
```

That tells the programmer what they want if they remember to look at the parameter name. Better than nothing, but still effectively nothing!

Maybe it would be more obvious if we make a type:

```rust
struct Radians(f32);

fn project_angle(start: (f32, f32), angle: Radians, radius: f32) -> (f32, f32) {
    (
        0.0 - (start.0 + radius * f32::sin(angle.0)),
        start.1 + radius * f32::cos(angle.0),
    )
}

fn main() {
    let start = (0.0, 0.0);
    let finish = project_angle(start, Radians(180.0f32.to_radians()), 10.0);
    println!("{finish:?}");
}
```

That's clearer, you at least can't avoid typing `Radians`---which should nudge people in the right direction. But our user *really* wants to use degrees sometimes.

So we make a `Degrees` type as well:

```rust
struct Radians(f32);

struct Degrees(f32);

impl Degrees {
    fn to_radians(&self) -> f32 {
        self.0.to_radians()
    }
}

fn project_angle(start: (f32, f32), angle: Radians, radius: f32) -> (f32, f32) {
    (
        0.0 - (start.0 + radius * f32::sin(angle.0)),
        start.1 + radius * f32::cos(angle.0),
    )
}

fn main() {
    let start = (0.0, 0.0);
    let finish = project_angle(start, Degrees(180.0).to_radians(), 10.0);
    println!("{finish:?}");
}
```

Again, it's better. We could add a reciprocal `ToDegrees` and have a decent API. But it's not a *smooth* API.

## Implementing `From`

We can do better. Let's implement Rust's `From` trait, which automatically sets up `into()` for types.

```rust
impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Radians {
        Radians(degrees.0.to_radians())
    }
}
```

Now we can replace the call with the much more "Rustacean":

```rust
let finish = project_angle(start, Degrees(180.0).into(), 10.0);
```

## Your First Generic Function

We can do even better than that, using generic functions. A generic function accepts one or more arguments as a trait, and only works for variables that implement that trait.

Let's modify our function again:

```rust
fn project_angle<A: Into<Radians>>(start: (f32, f32), angle: A, radius: f32) -> (f32, f32) {
    let angle: Radians = angle.into();
    (
        0.0 - (start.0 + radius * f32::sin(angle.0)),
        start.1 + radius * f32::cos(angle.0),
    )
}
```

We are:
* Adding a *trait requirement* to `project_angle`.
    * `<A: TRAIT>` defines parameter type A to have to match the listed trait.
    * Rust provides the `Into` trait - and automatically generates it when you create a `From` trait (but not vice versa)
    * So we're requiring that `Into<Radians>` exist for whatever type is sent as type `A`.
* We've changed `angle` to be of type `A`.

The call becomes:

```rust
let finish = project_angle(start, Degrees(180.0), 10.0);
```

> When making generic functions, it's worth remembering that they make your binary bigger and compilation slower. Rust will generate a version of `project_angle` for every angle type it encounters.

That's *much* better. You can't:
* Just send in `180.0` hoping you got it right.
* Send in a type that isn't `Degrees` or `Radians` (Rust automatically accepts `Into<myself>`).
* Accidentally miss your target because of unit confusion.

If you are working with multiple representations of the same thing, I recommend using the `NewType` pattern everywhere it can reduce confusion. That way, you can't accidentally be tripped up by millimeters, nanometers, decimeters and centimeters! It also makes it easier to support users in different locations who may want metric or imperial measurements.

## Another Useful Generic Function

While we're talking about generic functions, aren't you sick of typing `.to_string()` everywhere? You don't have to. `ToString` is a trait, and you can accept it in place of a `String` type.

[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=01d8848c8941b8e75c911ac218b0d683)

```rust
fn take_my_text<S: ToString>(text: S) {
    let s = text.to_string();
    // Work with the string
}

fn main() {
    take_my_text("Hello");
    take_my_text("Hello".to_string());
    take_my_text(String::new());
    let n = 5;
    take_my_text(n);
}
```

## Going back to strong typing...

Let's go back to where we were:

```rust
struct Radians(f32);

struct Degrees(f32);

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Radians {
        Radians(deg.0.to_radians())
    }
}

fn project_angle<A: Into<Radians>>(start: (f32, f32), angle: A, radius: f32) -> (f32, f32) {
    let angle: Radians = angle.into();
    (
        0.0 - (start.0 + radius * f32::sin(angle.0)),
        start.1 + radius * f32::cos(angle.0),
    )
}

fn main() {
    let start = (0.0, 0.0);
    let finish = project_angle(start, Degrees(180.0), 10.0);
    println!("{finish:?}");
}
```

Passing tuples around as coordinates isn't very pleasant, either. So let's make a `Point` type. With a `Point` type, it's quite a bit more readable:

```rust
struct Radians(f32);

struct Degrees(f32);

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Radians {
        Radians(deg.0.to_radians())
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn project_angle<A: Into<Radians>>(start: Point, angle: A, radius: f32) -> Point {
    let angle: Radians = angle.into();
    Point::new(
        0.0 - (start.x + radius * f32::sin(angle.0)),
        start.y + radius * f32::cos(angle.0),
    )
}

fn main() {
    let start = Point::new(0.0, 0.0);
    let finish = project_angle(start, Degrees(180.0), 10.0);
    println!("{finish:?}");
}
```

Let's go a step further, and allow points to be added to one another:

```rust
impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(mut self, rhs: Point) -> Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
```

We're implementing `std::ops::Add`. It takes a `type` binding, indicating that we are adding it to another `Point`. Then the `add` function is pretty straightforward.

Let's quickly check that it works:

```rust
let start = start + Point::new(1.0, 1.0);
```

You can implement all of the math operators if you want to. I find `+=` to be really useful, so let's add that one:

```rust
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
```

And test it:

```rust
fn main() {
    let start = Point::new(0.0, 0.0);
    let mut start = start + Point::new(1.0, 1.0);
    start += Point::new(1.0, 1.0);
    let finish = project_angle(start, Degrees(180.0), 10.0);
    println!("{finish:?}");
}
```

> What you *can't* do is randomly assign symbols to all new operations, or change the basic structure of what an operator does. What you *shouldn't* do is redefine "plus" to mean "minus", but Rust won't stop you.

