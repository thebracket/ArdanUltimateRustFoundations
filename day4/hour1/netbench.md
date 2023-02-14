# How Fast are our Network Services?

Let's add some timings to our network services, and look at a few ways we can improve things.

Let's open up the [TCP Login Server](/src/tcp_login_server/) program again.

> Live-coding. The benchmark version is [here](/src/tcp_login_server_bench/)

We'll break the `async_client` function into several functions to clearly separate out functionality. Here's a generic `request_login` function:

```rust
async fn request_login(username: &str, password: &str) -> anyhow::Result<LoginAction> {
    let login_attempt = LoginRequest {
        username: username.to_string(), 
        password: password.to_string(),
    };


    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    let message = bincode::serialize(&login_attempt)?;
    stream.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Option<LoginAction> = bincode::deserialize(&buf[0..n])?;


    match response {
        None => {
            Err(anyhow::Error::msg("Unknown User"))
        }
        Some(login_action) => {
            Ok(login_action)
        }
        _ => Ok(LoginAction::Denied(DeniedReason::AccountLocked { reason: "Unknown User".to_string() }))
    }
}
```

Let's start out with a very simple timed login request:

```rust
async fn rpc_client() -> anyhow::Result<()> {
    let now = std::time::Instant::now();
    let _result = request_login("herbert", "password").await?;
    let duration = now.elapsed();
    println!("Login session took: {} usecs", duration.as_micros());

    Ok(())
}
```

Running the server with `cargo run --release --server` and the client with `cargo run --release -- --client` yields:

```
Login session took: 2784 usecs
```

2,784 usecs is quite good. A single sample isn't very statistically relevant, so let's expand it to make 10 requests:

```rust
async fn rpc_client() -> anyhow::Result<()> {
    for _ in 0..10 {
        let now = std::time::Instant::now();
        let _result = request_login("herbert", "password").await?;
        let duration = now.elapsed();
        println!("Login session took: {} usecs", duration.as_micros());
    }

    Ok(())
}
```

There's a LOT of variance on our results:

```
Login session took: 4258 usecs
Login session took: 418 usecs
Login session took: 349 usecs
Login session took: 469 usecs
Login session took: 307 usecs
Login session took: 241 usecs
Login session took: 430 usecs
Login session took: 303 usecs
Login session took: 372 usecs
Login session took: 280 usecs
```

The first request is taking a lot longer than the others, despite each connection making a new TCP connection.

We can get some insight into *why* it the variance is present by spawning a separate green-thread per task:

```rust
async fn rpc_client() -> anyhow::Result<()> {
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(tokio::spawn(async {
            let now = std::time::Instant::now();
            let _result = request_login("herbert", "password").await.unwrap();
            let duration = now.elapsed();
            println!("Login session took: {} usecs", duration.as_micros());
        }));
    }
    for handle in handles {
        handle.await;
    }

    Ok(())
}
```

Now the results are consistent---sadly, consistent with the worst-case side of the range:

```
Login session took: 3327 usecs
Login session took: 3342 usecs
Login session took: 3354 usecs
Login session took: 3397 usecs
Login session took: 3401 usecs
Login session took: 3366 usecs
Login session took: 3435 usecs
Login session took: 3488 usecs
Login session took: 3556 usecs
Login session took: 3647 usecs
```

Spawning a green-thread is apparently costing a three thousand usecs of execution time. Let's see if we can prove that theory:

```rust
async fn rpc_client() -> anyhow::Result<()> {
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(tokio::spawn(async {
            for _ in 0..10 {
                let now = std::time::Instant::now();
                let _result = request_login("herbert", "password").await.unwrap();
                let duration = now.elapsed();
                println!("Login session took: {} usecs", duration.as_micros());
            }
        }));
    }
    for handle in handles {
        handle.await;
    }

    Ok(())
}
```

Now we're spawning 10 green-threads, each of which is performing 10 operations. The result:

```
ogin session took: 3810 usecs
Login session took: 4183 usecs
Login session took: 4226 usecs
Login session took: 4217 usecs
Login session took: 4246 usecs
Login session took: 4158 usecs
Login session took: 4272 usecs
Login session took: 4284 usecs
Login session took: 4289 usecs
Login session took: 4298 usecs
Login session took: 338 usecs
Login session took: 295 usecs
Login session took: 331 usecs
Login session took: 334 usecs
Login session took: 313 usecs
Login session took: 416 usecs
Login session took: 409 usecs
Login session took: 310 usecs
Login session took: 325 usecs
Login session took: 297 usecs
Login session took: 253 usecs
Login session took: 360 usecs
Login session took: 321 usecs
Login session took: 334 usecs
Login session took: 414 usecs
Login session took: 345 usecs
Login session took: 340 usecs
Login session took: 407 usecs
Login session took: 306 usecs
Login session took: 374 usecs
Login session took: 349 usecs
Login session took: 370 usecs
Login session took: 460 usecs
Login session took: 372 usecs
Login session took: 375 usecs
Login session took: 436 usecs
Login session took: 307 usecs
Login session took: 482 usecs
Login session took: 441 usecs
Login session took: 411 usecs
Login session took: 387 usecs
Login session took: 281 usecs
Login session took: 367 usecs
Login session took: 272 usecs
Login session took: 392 usecs
Login session took: 355 usecs
Login session took: 333 usecs
Login session took: 343 usecs
Login session took: 308 usecs
Login session took: 310 usecs
Login session took: 272 usecs
Login session took: 467 usecs
Login session took: 381 usecs
Login session took: 352 usecs
Login session took: 355 usecs
Login session took: 330 usecs
Login session took: 329 usecs
Login session took: 341 usecs
Login session took: 348 usecs
Login session took: 408 usecs
Login session took: 359 usecs
Login session took: 355 usecs
Login session took: 394 usecs
Login session took: 338 usecs
Login session took: 324 usecs
Login session took: 392 usecs
Login session took: 321 usecs
Login session took: 342 usecs
Login session took: 351 usecs
Login session took: 315 usecs
Login session took: 289 usecs
Login session took: 384 usecs
Login session took: 474 usecs
Login session took: 317 usecs
Login session took: 326 usecs
Login session took: 254 usecs
Login session took: 331 usecs
Login session took: 391 usecs
Login session took: 318 usecs
Login session took: 308 usecs
Login session took: 431 usecs
Login session took: 314 usecs
Login session took: 347 usecs
Login session took: 330 usecs
Login session took: 326 usecs
Login session took: 329 usecs
Login session took: 344 usecs
Login session took: 308 usecs
Login session took: 322 usecs
Login session took: 315 usecs
Login session took: 301 usecs
Login session took: 291 usecs
Login session took: 304 usecs
Login session took: 463 usecs
Login session took: 257 usecs
Login session took: 261 usecs
Login session took: 324 usecs
Login session took: 23399 usecs
Login session took: 867 usecs
Login session took: 666 usecs
```

The first connection in each green-thread takes a while, and subsequent connections are very fast. So lesson #1: batch your green threads!

Now let's see if we can re-use the TCP connection. It's surprisingly easy to create a structure that wraps a connected TCP stream:

```rust
struct LoginClient(TcpStream);

impl LoginClient {
    async fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:8123").await.unwrap();
        Self(stream)
    }
}
```

We can pretty much create the a `login` function with cut/paste:

```rust
async fn login(&mut self, username: &str, password: &str) -> anyhow::Result<LoginAction> {
    let login_attempt = LoginRequest {
        username: username.to_string(), 
        password: password.to_string(),
    };
    let message = bincode::serialize(&login_attempt)?;
    self.0.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = self.0.read(&mut buf).await?;
    let response: Option<LoginAction> = bincode::deserialize(&buf[0..n])?;


    match response {
        None => {
            Err(anyhow::Error::msg("Unknown User"))
        }
        Some(login_action) => {
            Ok(login_action)
        }
        _ => Ok(LoginAction::Denied(DeniedReason::AccountLocked { reason: "Unknown User".to_string() }))
    }
}
```

So with that in place, let's change our client to spawn a new connection per thread and then re-use it.

```rust
async fn rpc_client() -> anyhow::Result<()> {
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(tokio::spawn(async {
            let mut client = LoginClient::new().await;
            for _ in 0..10 {
                let now = std::time::Instant::now();
                let _result = client.login("herbert", "password").await.unwrap();
                let duration = now.elapsed();
                println!("Login session took: {} usecs", duration.as_micros());
            }
        }));
    }
    for handle in handles {
        handle.await;
    }

    Ok(())
}
```

The results are dramatically improved:

```
Login session took: 178 usecs
Login session took: 162 usecs
Login session took: 542 usecs
Login session took: 538 usecs
Login session took: 625 usecs
Login session took: 634 usecs
Login session took: 635 usecs
Login session took: 579 usecs
Login session took: 599 usecs
Login session took: 592 usecs
Login session took: 195 usecs
Login session took: 70 usecs
Login session took: 65 usecs
Login session took: 49 usecs
Login session took: 61 usecs
Login session took: 61 usecs
Login session took: 51 usecs
Login session took: 103 usecs
Login session took: 77 usecs
Login session took: 66 usecs
Login session took: 61 usecs
Login session took: 76 usecs
Login session took: 71 usecs
Login session took: 61 usecs
Login session took: 44 usecs
Login session took: 46 usecs
Login session took: 93 usecs
Login session took: 65 usecs
Login session took: 49 usecs
Login session took: 91 usecs
Login session took: 50 usecs
Login session took: 75 usecs
Login session took: 114 usecs
Login session took: 97 usecs
Login session took: 74 usecs
Login session took: 64 usecs
Login session took: 87 usecs
Login session took: 101 usecs
Login session took: 84 usecs
Login session took: 59 usecs
Login session took: 43 usecs
Login session took: 124 usecs
Login session took: 96 usecs
Login session took: 54 usecs
Login session took: 107 usecs
Login session took: 101 usecs
Login session took: 77 usecs
Login session took: 106 usecs
Login session took: 75 usecs
Login session took: 55 usecs
Login session took: 84 usecs
Login session took: 60 usecs
Login session took: 56 usecs
Login session took: 99 usecs
Login session took: 61 usecs
Login session took: 56 usecs
Login session took: 48 usecs
Login session took: 39 usecs
Login session took: 57 usecs
Login session took: 79 usecs
Login session took: 52 usecs
Login session took: 133 usecs
Login session took: 138 usecs
Login session took: 130 usecs
Login session took: 213 usecs
Login session took: 143 usecs
Login session took: 220 usecs
Login session took: 174 usecs
Login session took: 249 usecs
Login session took: 153 usecs
Login session took: 187 usecs
Login session took: 129 usecs
Login session took: 172 usecs
Login session took: 189 usecs
Login session took: 271 usecs
Login session took: 169 usecs
Login session took: 151 usecs
Login session took: 65 usecs
Login session took: 313 usecs
Login session took: 241 usecs
Login session took: 147 usecs
Login session took: 173 usecs
Login session took: 166 usecs
Login session took: 155 usecs
Login session took: 153 usecs
Login session took: 185 usecs
Login session took: 253 usecs
Login session took: 164 usecs
Login session took: 95 usecs
Login session took: 84 usecs
Login session took: 58 usecs
Login session took: 75 usecs
Login session took: 55 usecs
Login session took: 94 usecs
Login session took: 80 usecs
Login session took: 51 usecs
Login session took: 65 usecs
Login session took: 64 usecs
Login session took: 53 usecs
Login session took: 180 usecs
```

There's still some variance, but it seems that most of the wait time is creating the TCP connection! Individual requests can be as fast as 51 usec.

Let's change it once more, this time to use 1,000 threads.

```rust
async fn rpc_client() -> anyhow::Result<()> {
    let mut handles = Vec::new();
    for _ in 0..1_000 {
        handles.push(tokio::spawn(async {
            let mut client = LoginClient::new().await;
            for _ in 0..10 {
                let now = std::time::Instant::now();
                let _result = client.login("herbert", "password").await.unwrap();
                let duration = now.elapsed();
                println!("Login session took: {} usecs", duration.as_micros());
            }
        }));
    }
    for handle in handles {
        handle.await;
    }

    Ok(())
}
```

We get a *lot* of results---with a lot more variance. The worse case is still in the 20000usec range (0.02 seconds - still fast). So adding load definitely affects overall performance (as expected), but it's not disastrous.

Let's try 100,000 requests. Now we start to see the dreaded:

```
called `Result::unwrap()` on an `Err` value: Os { code: 10061, kind: ConnectionRefused, message: "No connection could be made because the target machine actively refused it." }Login session took: 34287 usecs 
'
```

The login server simply can't keep up with 100,000 TCP connections. Nothing can, given that there's only 65,535 ports available.

We'll dive more into this if you take the Optimization class. For now, though---you've got a very performant setup. Just remember that *making the connection* is often far more expensive than actually doing the work, especially if you are in an environment with "very micro" services ("nanoservices?").