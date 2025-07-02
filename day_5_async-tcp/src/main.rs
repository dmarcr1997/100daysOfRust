use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("👻GhostShell Running on PORT:3000");
    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            println!("👾Connection from {:?}", addr);
            loop {
                let mut buf = [0; 1024];
                let bytes_read = match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        break;
                    }
                };
                if let Err(e) = socket.write_all(&buf[..bytes_read]).await {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                    break;
                }
            }
        });
    }
}
