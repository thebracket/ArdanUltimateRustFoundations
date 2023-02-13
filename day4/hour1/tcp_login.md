# Putting it all together - a TCP Login Server

> We're working with the Github repo [here](/src/tcp_login_server/)

We've covered a lot of this before, so we'll skim over the initial setup a bit.

We'll start a new project, add it to our workspace, and add all the dependencies from our [RPC Login](/day2/hour4/tcp_server.md) example.

```toml
[dependencies]
anyhow = "1.0.69"
serde = { version = "1.0.152", features = ["derive"] }
serde_json = "1.0.93"
tokio = { version = "1.25.0", features = ["full"] }
bincode = "1"
```

There's a new one - `bincode`. Bin-code is an efficient bit-packer that packs bits tightly together into binary. Rather than waste network resources on large JSON files, we'll use tightly encoded binary. This will let you see how easy it is to use `Serde` with multiple data types.

Let's also add our `authentication` project to our dependencies:

```toml
authentication = { path = "../authentication" }
```

Finally, we'll add `once_cell` (which you learned about in Global Variables), and `parking_lot` (which you learned about in `parking_lot`):

```toml
once_cell = "1"
parking_lot = "0"
```

Now we take out `main.rs` file and paste in the previous RPC login code:

```rust
use serde::{Serialize, Deserialize};
use tokio::{net::{TcpListener, TcpStream}, spawn, io::{AsyncReadExt, AsyncWriteExt}};

#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Error,
    Ack,
}

async fn rpc_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                
                if n == 0 {
                    return;
                }

                let mut response = Response::Error;
                let request = serde_json::from_slice(&buf[0..n]);
                match request {
                    Err(..) => return,
                    Ok(request) => {
                        match request {
                            Request::Ping => response = Response::Ack,
                        }
                    }
                }

                let bytes = serde_json::to_vec(&response).unwrap();
                socket
                    .write_all(&bytes)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
    Ok(())
}

async fn rpc_client() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    let message = serde_json::to_vec(&Request::Ping)?;
    stream.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Response = serde_json::from_slice(&buf[0..n])?;
    match response {
        Response::Error => println!("Error!"),
        Response::Ack => println!("Ack"),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("You must run with either --server or --client");
    } else {
        match args[1].as_str() {
            "--server" => rpc_server().await?,
            "--client" => rpc_client().await?,
            _ => println!("You must run with either --server or --client"),
        }
    }
    Ok(())
}
```

Also copy over the `users.json` file we created earlier.

At the top of `main.rs`, add an import for it:
```rust
use authentication::*;
```

First step: delete the `Request` and `Response` types. We're not simulating anything here, we're implementing an actual login server. Lots of errors appear---but that's ok, we'll get to them.

Now we want a safe, global store for user data. We can use our `get_users()` function from the `login` project on day 1. We're doing to safely wrap it in a `Lazy` `HashMap`:

```rust
static USERS: Lazy<RwLock<HashMap<String, User>>> = Lazy::new(|| RwLock::new(get_users()));
```

Next, let's define a type to hold a login request:

```rust
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
```

Now we'll change our request handler to look for a Login structure:

```rust
let mut response = None;
if let Ok(request) = bincode::deserialize::<LoginRequest>(&buf[0..n]) {
    response = login(&USERS.read(), &request.username, &request.password);

}

let bytes = bincode::serialize(&response).unwrap();
socket
    .write_all(&bytes)
    .await
    .expect("failed to write data to socket");
```

We've used the TurboFish to specify that we want bincode to deserialize into a `LoginRequest` type. Then we grab a `read` lock on the users list (which will be uncontested, since we aren't writing after initialization) and call our old `login` function with the username and password we received. We also change the result to be a `bincode` serialized byte stream.

That's the server done! Let's rewrite the client to do a network-based authentication:

```rust
async fn rpc_client() -> anyhow::Result<()> {
    println!("Welcome to the (Not Very) Secure Server");
    println!("Enter your username:");
    let mut username = String::new();
    let mut password = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut username).unwrap();
    println!("Enter your password:");
    stdin.read_line(&mut password).unwrap();

    let login_attempt = LoginRequest {
        username, password
    };


    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    let message = bincode::serialize(&login_attempt)?;
    stream.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Option<LoginAction> = bincode::deserialize(&buf[0..n])?;


    match response {
        None => {
            println!("{} is not a known user.", login_attempt.username.trim());
            println!("This is where we handle new users.");
        }
        Some(login_action) => {
            login_action.do_login(
                |user| println!("Welcome {user:?}"), 
                |reason| {
                    println!("Access denied");
                    println!("{reason:?}");
                }
            )
        }
    }

    Ok(())
}
```

We've copy/pasted the "ask for user/password" section from our original `login` program, as well as the result handling. Then we've just replaced the serialization/deserialization with a binary-encoded login request.

And... it works!

```
Enter your username:
herbert
Enter your password:
password
Welcome Admin
```

In this section, we've pulled together/synthesized a lot of what we've covered before:

* We've reused our `Login` crate.
* We've added `bincode` binary encoding for efficient network access.
* We've used `parking_lot` to reduce the amount of typing required on locks.
* We've used `Lazy` from `once_cell` to create a global user pool for the server.

All in 208 lines of code.