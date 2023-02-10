# Channels

Communicating between async tasks or threads is accomplished with *channels*. These are similar to the way Go lets you send data between go-routines.

> Live-Coding. The Github version is [here](/src/tokio_channels/)

Let's modify our `main` function to include a channel:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel
    let (tx, mut rx) = mpsc::channel(32); // 32 = buffer size

    spawn(rpc_server());
    spawn(rpc_client(rx));

    for _ in 0..10 {
        sleep(Duration::from_secs(1)).await;
        let _ = tx.send(1).await;
    }
    let _ = tx.send(2).await;

    Ok(())
}
```

* `mpsc` stands for "Multi-producer, single consumer". You can have lots of systems sending *into* the channel, and only one *receiving* from it.
* We create the channel with a buffer size of 32. That's really overkill.
* We sleep 10 times, and after each nap we use `tx.send()` to send the number 1 to whomever has the receiver.
* We've passed the receiver to the `rpc_client`.

Now let's modify the `rpc_client` function to receive messages, and only talk to the server when asked to do so.

```rust
async fn rpc_client(mut rx: Receiver<u32>) -> anyhow::Result<()> {
```

Note that `Receiver` is actually: `tokio::sync::mpsc::Receiver`.

Here's the function:

```rust
async fn rpc_client(mut rx: Receiver<u32>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;

    while let Some(n) = rx.recv().await {
        let message = serde_json::to_vec(&Request::Ping)?;
        stream.write_all(&message).await?;
    
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let response: Response = serde_json::from_slice(&buf[0..n])?;
        match response {
            Response::Error => println!("Error!"),
            Response::Ack => println!("Ack"),
        }       
    }

    Ok(())
}
```

We've added `rx.recv` to listen to messages, and send a `Ping` whenever we receive a message. Running it shows a steady stream of Acks.

## Broadcast Channels

> Live-Coding. The Github version is [here](/src/tokio_channels2/)

Let's change our `main` function to create a broadcast channel and spawn 10 clients, all of whom subscribe to it:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel
    let (tx, _rx) = tokio::sync::broadcast::channel::<u32>(32);
    spawn(rpc_server());
    for _ in 0..10 {
        spawn(rpc_client(tx.subscribe()));
    }

    for _ in 0..10 {
        sleep(Duration::from_secs(1)).await;
        let _ = tx.send(1);
    }

    Ok(())
}
```

Now we change `rpc_client` to accept the different type:

```rust
async fn rpc_client(mut rx: tokio::sync::broadcast::Receiver<u32>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;

    loop {
        let _n = rx.recv().await?;
        let message = serde_json::to_vec(&Request::Ping)?;
        stream.write_all(&message).await?;

        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let response: Response = serde_json::from_slice(&buf[0..n])?;
        match response {
            Response::Error => println!("Error!"),
            Response::Ack => println!("Ack"),
        }       
    }

    Ok(())
}
```

* We've changed the receiver type. 
* We've simplified it a bit by just awaiting `rx.recv`, 
* and complicated it again by switching to `?` instead of opening an option. *
* Otherwise, it's the same.

Now when you run the program, you see a burst of Acks. You have 10 clients each receiving the instruction to send a ping, and each does so - and reports the Ack.

## You don't have to use Tokio to use Channels

> Live-Coding. The Github version is [here](/src/thread_channels/)

Here's a threaded version:

```rust
use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    let handle = thread::spawn(move || {
        loop {
            let n = rx.recv().unwrap();
            match n {
                1 => println!("Hi from worker thread"),
                _ => break,
            }
        }
        println!("Thread closing cleanly");
    });

    for _ in 0..10 {
        tx.send(1).unwrap();
    }
    tx.send(0).unwrap();

    handle.join().unwrap();
}
```

The semantics are basically the same: we create an `mpsc::channel` (this time there is no capacity). We send the receiver into the thread, and match on its results. For a "1", we print something. Otherwise, we quit the thread. Then we send 10 messages, followed by a quit message.

The result is what you'd expect:

```
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Hi from worker thread
Thread closing cleanly
```

> There is no `broadcast` type in default Rust. The `crossbeam` crate provides some very high-quality channels.
