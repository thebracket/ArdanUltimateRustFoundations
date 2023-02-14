use std::collections::HashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use tokio::{net::{TcpListener, TcpStream}, spawn, io::{AsyncReadExt, AsyncWriteExt}};
use auth_json::*;

static USERS: Lazy<RwLock<HashMap<String, User>>> = Lazy::new(|| RwLock::new(get_users()));

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
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

                let mut response = None;
                if let Ok(request) = bincode::deserialize::<LoginRequest>(&buf[0..n]) {
                    response = login(&USERS.read(), &request.username, &request.password);

                }

                let bytes = bincode::serialize(&response).unwrap();
                socket
                    .write_all(&bytes)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
    Ok(())
}

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

struct LoginClient(TcpStream);

impl LoginClient {
    async fn new() -> Self {
        let stream = TcpStream::connect("127.0.0.1:8123").await.unwrap();
        Self(stream)
    }

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
}

async fn rpc_client() -> anyhow::Result<()> {
    let mut handles = Vec::new();
    for _ in 0..100_000 {
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
