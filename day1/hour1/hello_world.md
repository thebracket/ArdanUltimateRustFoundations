# First Program: Hello World

> This section will be demonstrated live. You are encouraged to follow along. You can see *most* of this online at replit.com: [https://replit.com/@HerbertWolverso/HelloWorld#src/main.rs](https://replit.com/@HerbertWolverso/HelloWorld#src/main.rs)

## Create a new Rust Project

> I find it helpful to have a copy of Visual Studio Code running with this repo, and a *second* window for the code-along examples.

1. Open VSCode.
2. Select "Open Folder" and load wherever you cloned this GitHub repo.
    * If you didn't clone it, you can select "Clone Git Repository" and enter `https://github.com/thebracket/ArdanUltimateRustFoundations.git`
3. Select `File` -> `New Window`.
4. Open the integrated terminal with `View` -> `Terminal` or `Ctrl` + `backtick`

Let's make a new Rust program with `cargo new`:

1. Change to a new directory. For the demo, I'll be using `C:\users\herbert\rust\live`. Type: `cd rust/live` `<enter>`
2. Create a new project with `cargo new login_system`.
    * `login_system` is the name of the program to create.

> You can also use `cargo init`. They do the same thing. I've been using Rust since before `new` became the standard, so I may occasionally type `init` without thinking!

You should see:

```
     Created binary (application) `login_system` package
```

Let's take a quick tour of everything that has been done. There's some hidden stuff here!

```
cd login_system
tree /f 
    (on Windows), du -h will give a similar display on *NIX.
```

There's more here than you might expect:

```
C:.
│   .gitignore
│   Cargo.toml
│
└───src
        main.rs
```

## Surprise: Git Integration!

> Rust generally tries to avoid surprises. I was surprised when I discovered that git integration was the default!

* `.gitignore`. Rust has created a Git file to automatically exclude your `target` directory (where compilation occurs).
* If you `dir -h .`, you discover a `.git` folder and created a whole new Git repository!

That's great, but you may not always want to do that. You can NOT use Git by modifying the `cargo new` command:

```
cargo new --vcs none login_system_nogit
```

> I changed the project name to include "_nogit" - you don't have to do that.

If your organization doesn't use git, there are some other source control options built in:

* [git](https://git-scm.com/) - the default, using Git.
* [hg](https://www.mercurial-scm.org/)
* [pijul](https://pijul.org/)
* [fossil](https://www2.fossil-scm.org/home/doc/trunk/www/index.wiki)
* `none` - don't do any automatic SCM integration.

If you are using a different source control manager (such as `Subversion`), make you to exclude your **target** directory from being committed.

> **Tip**: If you are already inside a Git workspace, creating a new project won't make another one.

## Cargo.toml

Every new project will have a `Cargo.toml` file created. Here's an annotated version of the default file:

```toml
[package]               # Every Cargo project needs a package section
name = "login_system"   # The project name, which will double as the target filename (e.g. login_system.exe)
version = "0.1.0"       # Version in semantic versioning. We'll deal more with versions later.
edition = "2021"        # Which edition of Rust. Usually 2021, may be older for some packages.

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Defaults to an empty list. This is where you add dependencies, which we'll do later.

```

Why 2021? Every few years, Rust issues a new *edition*. This is the only time at which substantial syntax changes are permitted. Rust retain compatibility with older editions (and syntaxes) by specifying the edition to use when building.

## The `src` folder

Your source code largely lives in the `src` folder. Rust has created a `main.rs` file for you.

* If a project has a `main.rs`, it expects to have a function named `main()` - and builds an executable.
* If a project has a `lib.rs` file, it's a library. We'll make one in a moment.
* If a project has *both*, then it can be included in other projects as a library - or executed. This requires a bit of redundant typing, but is great for attaching unit tests, benchmarks and other *library* features to something you can also execute.

Let's take a quick look at `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

There's only a little here to note:

* Rust is *terse* - `fn` for `function`, very little space is wasted.
* `fn main()` creates a new *function* called `main`, with no parameters/arguments.
* `println!` has a mysterious `!`. That denotes it as a *macro*. You'll write macros later in this class. For now, a *macro* is anything that uses the in-built macro language to support syntax that doesn't fit inside regular Rust syntax.
* Click on `println` and open your "Command Palette". Search for "expand" and find "Expand Macro Recursively" (a Rust Analyzer feature).

![](/images/ExpandMacro.png)

Once you run this command, you can find out what `println!` actually does. It's remarkably complicated:

```rust
{
    $crate::io::_print($crate::fmt::Arguments::new_v1(&[], &[]));
}
```

We'll talk about macros more at the [end of this class](/day4/hour1/macros.md).

## Run "Hello World"

You can run the newly created program with `cargo run`.

Predictably, you'll see the output:

```
   Compiling hello_login_system v0.1.0 (C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\src\hello_login_system)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\hello_login_system.exe`   
Hello, world!
```

Since we're here, it's a good time to note that this runs everything in *debug* mode. Optimizations are disabled, extra debug information is generated, and extra run-time error checks are running.

You can always skip these checks and run in optimized mode with:

```
cargo run --release
```

(Which gives the same output with a slower compilation and un-noticeably faster execution for such a tiny program)

> You can find the "hello world" example [here](/src/hello_login_system/).