# Make a TCP Server with Tokio

Tokio and async are a great fit for network servers. A Tokio server can be really small, making it work well for microservices---or really big, managing the core of a giant macro-service.

> Live-coding. Github version is [here](/src/tokio_tcp/)

Let's go back to "Hello, Tokio":

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    Ok(())
}
```

Tokio has a lot of networking support built-in. We'll make use of that to build a simple TCP echo server - it replies back with whatever you said to it.

The first step is to bind a TCP socket and listen for connections:

```rust
let listener = TcpListener::bind("127.0.0.1:8123").await?;
```

We're using `?` to bail out with the wrapped error if it goes wrong. It's an async function, so we `await` the result.

Next, we want to accept connections from the TCP Listener. We're going to do this in an infinite loop---we want to process whatever connections may arrive:

```rust
loop {
    let (mut socket, address) = listener.accept().await?;
}
```

So in a loop, we are waiting for a connection. If one arrives, we receive the address and a `TcpStream`---which represents text going back and forth in the TCP connection.

**Important: we don't want to wait for this connection to finish to handle the next one!**

So we *spawn* a new green-thread to handle talking to *this connection*:

```rust
spawn(async move {

});
```

This is the Tokio `spawn` function from the previous section, but we're calling a *closure*. You can make closure `async` with the `async` keyword.

Now we need a buffer into which we will read data, and a loop similar to the one you used to read from the keyboard:

```rust
spawn(async move {
    let mut buf = vec![0; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");
    }
});
```

So we're spawning a new green-thread, moving `socket` from the parent task into the new task. Then we call `read` on the socket, and wait for it to receive some data. It will return the number of bytes it received.

Let's quit if there is no data:

```rust
if n == 0 {
    return;
}
```

We're quitting this task - the server stays up!

Finally, let's reply back with whatever was said to the server:

```rust
socket
    .write_all(&buf[0..n])
    .await
    .expect("failed to write data to socket");
```

Let's run the program with `cargo run` and connect "PuTTY" to `localhost` port `8123`:

![](/images/PuttyEcho.png)

Whatever we type gets echoed back to us.

## Let's Make an RPC Server

> Live-Coding. The Github repo for this example is [here](/src/tokio_rpc/)

Let's add a couple of packages we used yesterday:

```
cargo add serde -F derive
cargo add serde_json
```

Now we'll add two serializable types:

```rust
#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Error,
    Ack,
}
```

This is just like what we did [yesterday](../../day1/hour3/serialization.md).

Let's refactor the `main` function and create an `rpc_server` function:

```rust
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

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
    Ok(())
}
```

We've literally moved `main` into this function. Now we'll replace the `write_all` portion that echoes back with some logic to use Serde on the incoming request.

```rust
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
```

We are:

1. Creating a default response (error).
2. Asking Serde to deserialize the incoming buffer, directly from a slice of bytes. There's some coolness under the hood that avoids copying.
3. Using `match` to cancel the socket if the data was unreadable, and read the result if all is well.
4. If we see the `Ping`, we change the response to an `Ack`.

Now we need to send the response.

```rust
let bytes = serde_json::to_vec(&response).unwrap();
socket
    .write_all(&bytes)
    .await
    .expect("failed to write data to socket");
```

Now lets change `main` to start the server if "--server" was present on the command-line. We'll skip `clap`, it's overkill for this.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        println!("You must run with either --server or --client");
    } else {
        match args[0].as_str() {
            "--server" => rpc_server().await?,
            "--client" => rpc_client().await?,
            _ => println!("You must run with either --server or --client"),
        }
    }
    Ok(())
}

```

We haven't written the client yet---so let's do that.

```rust
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
```

We can use `TcpStream` as a client. We:

1. Connect to the remote TCP server.
2. Serialize a `Ping` message.
3. Send the bytes to the remote TCP server.
4. Use `read` to get what the other side has to say.
5. Try to decode the response.
6. Print the result.

Let's give it a go. You'll want to run one copy (`cargo run -- --server`) in one window, and the other (`cargo run -- --client`) in another.

The server runs, and the client sees an `Ack`.

Congratulations---you've made a simple RPC server and client in 84 lines of code!