use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (mut sock, _) = listener.accept().await?;
        println!("Connection from {:?}", sock);

        tokio::spawn(async move {
            let (mut reader, mut writer) = sock.split();
            match tokio::io::copy(&mut reader, &mut writer).await {
                Ok(n) => {
                    println!("Wrote {} bytes", n);
                }
                Err(err) => {
                    eprintln!("copy() error: {:?}", err);
                }
            }
        });
    }
}
