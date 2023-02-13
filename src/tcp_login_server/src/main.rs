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
