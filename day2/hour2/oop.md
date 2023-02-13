# Unlearning some Object Oriented Patterns

> We're live coding. The Github for [this section](/src/borrow_oops/)

Coming from an object-oriented language, or even a `NoSQL` world, you may be really tempted to arrange your data like this:

```rust
struct Organization {
    pub people: Vec<Person>,    
}

struct Person {
    pub resources: Vec<Resource>,
}

impl Person {
    fn give_resource(&mut self, name: &str, org: &mut Organization, recipient: usize) {
        if let Some((idx, resource)) = self.resources.iter().enumerate().find(|(_, item)| name == item.name) {
            self.resources.remove(idx);
            org.people[recipient].resources.push(resource.clone());
        }
    }
}

#[derive(Clone)]
struct Resource {
    pub name: String,
}

fn main() {
    let mut org = Organization {
        people: vec![
            Person { resources: vec![ Resource { name: "Stapler".to_string() } ]},
            Person { resources: Vec::new() },
        ]
    };
    org.people[0].give_resource("Stapler", &mut org, 1);
}

```

It looks reasonable, and it aligns with a lot of what you're taught in school:
* You represent your organization with a type.
* That type owns *people* who work for it.
* Each *person* may have resources.

You give the stapler to person 0, and have a nice OOP-readable command to give the stapler to person 1.

**BAD NEWS: This won't compile!**

```
error[E0502]: cannot borrow `self.resources` as mutable because it is also borrowed as immutable
  --> src\borrow_oops\src\main.rs:12:13
   |
11 |         if let Some((idx, resource)) = self.resources.iter().enumerate().find(|(_, item)| name == item.name) {
   |                                        --------------------- immutable borrow occurs here
12 |             self.resources.remove(idx);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
13 |             org.people[recipient].resources.push(*resource);
   |                                                  --------- immutable borrow later used here

error[E0507]: cannot move out of `*resource` which is behind a shared reference
  --> src\borrow_oops\src\main.rs:13:50
   |
13 |             org.people[recipient].resources.push(*resource);
   |                                                  ^^^^^^^^^ move occurs because `*resource` has type `Resource`, which does not implement the `Copy` trait

error[E0499]: cannot borrow `org` as mutable more than once at a time
  --> src\borrow_oops\src\main.rs:29:44
   |
29 |     org.people[0].give_resource("Stapler", &mut org, 1);
   |     ----------    -------------            ^^^^^^^^ second mutable borrow occurs here
   |     |             |
   |     |             first borrow later used by call
   |     first mutable borrow occurs here

Some errors have detailed explanations: E0499, E0502, E0507.
For more information about an error, try `rustc --explain E0499`.
error: could not compile `borrow_oops` due to 3 previous errors
```

That's quite the error. The ultimate problem is that we've violated the golden rule of the borrow checker: *you can never borrow something mutably and also have immutable access to it.*

> We're live coding. The Github for [this section](/src/borrow_oops2/)

To make the same thing work with Rust, you have to do two things:
* Break the problem down into parts.
* Move the function so that it is an *organization* function---only requiring one ownership, while the organization retains ownership of people, so it can modify them.

This compiles:

```rust
struct Organization {
    pub people: Vec<Person>,    
}

impl Organization {
    fn move_resource(&mut self, from: usize, to: usize, name: &str) {
        if let Some(resource) = self.people[from].take_resource(name) {
            self.people[to].give_resource(resource);
        }
    }
}

struct Person {
    pub resources: Vec<Resource>,
}

impl Person {
    fn take_resource(&mut self, name: &str) -> Option<Resource> {
        let index = self.resources.iter().position(|r| r.name == name);
        if let Some(index) = index {
            let resource = self.resources.remove(index);
            Some(resource)
        } else {
            None
        }
    }

    fn give_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
}

// #[derive(Clone)] // We don't need this anymore
struct Resource {
    pub name: String,
}

fn main() {
    let mut org = Organization {
        people: vec![
            Person { resources: vec![ Resource { name: "Stapler".to_string() } ]},
            Person { resources: Vec::new() },
        ]
    };
    org.move_resource(0, 1, "stapler");
}
```

So what did we do here? We actually made a better program overall, because ownership is never in question and we stop doing multiple things at once---which makes each step *testable*, and gives you better options for handling errors.

The `Organization` owns its workers and their resources (hopefully not literally). As such, a single mutable borrow gives you the ability to give the `Organization` orders---without having to try and index the organization from within at a granular level.

When you call `move_resource`, you've broken the operation down into a two-step operation:
1. You call `take_resource` on a person. This encourages you to check that the person *has* the resource, and lets you act if that's not the case.
2. Once you are *sure* that the person no longer has the resource, you've *moved* it to the organization. The `remove` function returns the removed structure - so you can hand it off with a move.
3. Now that you *know* you've got the only copy of the resource, you can *move* it to the new recipient.

The *move* semantic matches reality: there's only one red Swingline stapler, and you know where it is at all times.

In a real application, you'd want to add error checking:
* Does person 1 exist?
* Does person 2 exist?
* What if adding to person 2 fails?

Adding that would have been a mess with the initial, condensed setup. Rust is forcing you to be methodical.

> We're live coding. The Github for [this section](/src/borrow_oops3/)

## Cleaning Up and Adding Error Handling

Let's clean this up into some robust code. 

Start by adding `thiserror` to your `Cargo.toml`:

```toml
[package]
name = "borrow_oops3"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
thiserror = "1"
```

Then we'll start coming up with all the things that could go wrong and define them:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum OrgError {
    #[error("The requested person does not exist")]
    PersonDoesNotExist(usize),
    #[error("The requested resource is not allocated to the person")]
    ResourceNotFound,
}
```

We'll then by no-longer using indices directly, and adding `id` numbers to people:

```rust
struct Person {
    id: usize,
    pub resources: Vec<Resource>,
}
```

And in our initialization:

```rust
people: vec![
    Person { id: 0, resources: vec![ Resource { name: "Stapler".to_string() } ]},
    Person { id: 1, resources: Vec::new() },
]
```

Then we'll add a function to `Organization` to find people:

```rust
fn find_person(&self, id: usize) -> Option<usize> {
    self.people.iter().position(|p| p.id == id)
}
```

That lets us add an immediate safety check that people exist:

```rust
impl Organization {
    fn move_resource(&mut self, from: usize, to: usize, name: &str) -> Result<(), OrgError> {
        if let (Some(id1), Some(id2)) = (self.find_person(from), self.find_person(2)) {
            if let Some(resource) = self.people[id1].take_resource(name) {
                self.people[id2].give_resource(resource);
                Ok(())
            } else {
                Err(OrgError::ResourceNotFound)
            }
        } else if self.find_person(from).is_none() {
            return Err(OrgError::PersonDoesNotExist(from));
        } else {
            return Err(OrgError::PersonDoesNotExist(to));
        }
    }
    ..
```

Now our `main` function includes the possibility of error:

```rust
org.move_resource(0, 1, "stapler").unwrap();
```

Unwrapping the "simple" OOP operation into constituent parts has:
* Removed the borrow checker error.
* Made it much more obvious what the code does by taking it one step at a time.
* Made the code more robust by giving you points from which you can flag specific errors.

Usually, moving from traditional Object-Oriented operations into Rust-land follows this pattern: move ownership of the operation to a level that *actually owns* all of the parts, and break the work down into simple pieces.