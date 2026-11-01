use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip: &str = "127.0.0.1";
    let port: &str = "6379";

    let address: String = format!("{}:{}", ip, port);
    let listener = TcpListener::bind(address).await?;

    println!("Server started on ip: {}\nlistening on port: {}", ip, port);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {

            let mut buf: [u8; 1024] = [0; 1024];

            loop {
                let raw_socket: usize = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let request = String::from_utf8_lossy(&buf[..raw_socket]);
                println!("{} {:?}", raw_socket, request);

                match request.to_lowercase().as_str() {
                    "*1\r\n$4\r\nping\r\n" => ping(&mut socket).await,
                    "*1\r\n$3\r\nset\r\n" => data_set(&mut socket).await,
                    _ => {
                        if let Err(e) = socket.write_all(b"-ERRunknown command\r\n").await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                };
            }
        });
    }
}

async fn ping(socket: &mut TcpStream) {
    if let Err(e) = socket.write_all(b"+PONG\r\n").await {
        eprintln!("failed to write to socket; err = {:?}", e);
    }
}

async fn data_set(socket: &mut TcpStream) {
    if let Err(e) = socket.write_all(b"+set\r\n").await {
        eprintln!("failed to write to socker; err = {:?}", e);
    }
}
