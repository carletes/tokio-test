use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("Created stream");

    let result = stream.write(b"Hello, world!\n").await?;
    println!("Wrote to stream. Result: {:?}", result);

    Ok(())
}
