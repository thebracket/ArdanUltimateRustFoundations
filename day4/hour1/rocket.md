# A Simple Web Application

We're going to use `Rocket` as a webserver. Not because it's the best, but because it's simple and fast--and fits in the remaining time.

## Hello Web

> We're live coding. The GitHub source is [here](/src/rocket/)

Create a new project, named `web`. Add it to your workspace.

Open `Cargo.toml` and add the following dependencies:

```toml
[dependencies]
rocket = "0.5.0-rc.2"
```

Let's paste the following into our `main.rs` file:

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

Run it with `cargo run` (it may take a minute to compile), and you will see:

```
     Running `C:\Users\Herbert\Documents\Ardan\Rust Foundations 4 Day\target\debug\rocket.exe`
Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 20
   >> ident: Rocket
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: C:\Users\Herbert\AppData\Local\Temp\
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
Routes:
   >> (index) GET /
Fairings:
   >> Shield (liftoff, response, singleton)
Shield:
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
Rocket has launched from http://127.0.0.1:8000
```

Open a browser window and point to [http://127.0.0.1:8000](http://127.0.0.1:8000). You should see a webpage that just reads `Hello, world!`.

Rocket lets you get up and running *fast* with a web-server.

## Serving up Some Content

Let's add a simple login page. In the `main.rs` file we build a handler, and a route --- just like you would in `nodejs` or similar:

```rust
#[macro_use] extern crate rocket;
use rocket::fs::NamedFile;

#[get("/")]
pub async fn login_page<'a>() -> NamedFile {
  NamedFile::open("login.html").await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login_page])
}
```

We'll also need some content to serve up. JavaScript isn't really my friend, so here's a quick and dirty login form with some jQuery:

```javascript
<html>
    <head>
        <title>Please Login</title>
        <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.6.3/jquery.min.js"></script>
    </head>
    <body>
            <label for="username">Username:</label>
            <input id="username" />
            <label for="password">Password:</label>
            <input type="password" id="password" />
            <button id="doLogin">Login</button>

            <script>                
                $("#doLogin").on('click', () => {
                    let newUser = {
                        username: $("#username").val(),
                        password: $("#password").val()
                    };
                    console.log(newUser);
                    $.ajax({
                        type: "POST",
                        url: "/api/login",
                        data: JSON.stringify(newUser),
                        success: (data) => {
                            if (data == "ERROR") {
                                alert("Invalid login")
                            } else {
                                window.location.href = "/";
                            }
                        }
                        })
                })
            </script>
    </body>
</html>
```

Run the program (`cargo run`), and connect to [http://localhost:8000/](http://localhost:8000/). You should see a login page, albeit an ugly one!

If you look at the JavaScript, we're posting a JSON form to `/api/login`. Let's create that on the Rust side. We'll start by making the login form type:

```rust
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Login {
    username: String,
    password: String,
}
```

The only part that annoys me here is that you are using `Serde` built-into Rocket, and it requires that you give it the path to where it can find Serde.

```rust
use rocket::serde::{json::Json, Deserialize, Serialize};

#[post("/api/login", data = "<user>")]
pub fn login(user: Json<Login>) {
    println!("{user:?}");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login_page, login])
}
```

Run the program and try to login. On the console, you'll see that Rocket/Serde have deserialized your posted data:

```
Json(Login { username: "herbert", password: "password" })
```

## Wire it to the TCP login service

You need to add `bincode` to your `Cargo.toml` dependencies. You'll also need your authentication service:

```toml
bincode = "1"
authentication = { path = "../authentication" }
```

Because Rocket already includes Tokio, it's quite simple to paste in our login code from the TCP Login Server example:

```rust
#[post("/api/login", data = "<user>")]
pub async fn login(user: Json<Login>) {
    use rocket::tokio::io::{AsyncWriteExt, AsyncReadExt};
    use rocket::tokio::net::TcpStream;

    use auth_json::*;
    let login_attempt = user.0;

    let mut stream = TcpStream::connect("127.0.0.1:8123").await.unwrap();
    let message = bincode::serialize(&login_attempt).unwrap();
    stream.write_all(&message).await.unwrap();

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response: Option<LoginAction> = bincode::deserialize(&buf[0..n]).unwrap();

    println!("{response:?}");
}
```

> Don't forget to run the TCP Login Server in server mode!

Now run the program, fill out the form, and see that the result printed is: `Some(Accept(Admin))`

You've successfully implemented a miniature micro-service setup. You have a user-facing web server, that receives JSON from the client. It then encodes it, sends it off to the TCP server, and receives a response.