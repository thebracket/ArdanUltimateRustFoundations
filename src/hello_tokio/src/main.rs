use std::time::Duration;

use tokio::{join, spawn, task::spawn_blocking};

async fn hello(n: u32) {
    println!("Hello {n}");
    if n < 10 {
        spawn(hello_child(n*10));
    }
}

async fn hello_child(n: u32) {
    println!("Hello again {n}");
    let _ = spawn_blocking(|| std::thread::sleep(Duration::from_secs(1))).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    join!(
        hello(1), hello(2), hello(3), hello(4)
    );
    Ok(())
}
