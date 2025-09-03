use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080..");
    loop {
        let (stream, _) = listener.accept().await?;
        println!("Accepted a new connection!");
    }
}
