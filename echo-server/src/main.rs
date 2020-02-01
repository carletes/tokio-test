use futures::stream::StreamExt; // Why not `tokio::stream::StreamExt`?
use tokio::net::TcpListener;
// use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            match conn {
                Ok(mut sock) => {
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
                Err(err) => {
                    eprintln!("accept() error: {:?}", err);
                }
            }
        }
    };

    println!("Server listening on {}", addr);
    server.await;
}
