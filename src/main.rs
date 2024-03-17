use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::sync::Arc;
use std::io;
use std::io::Error;

// async fn handle_client(stream: async_std::net::TcpStream) {
//     let mut stream = stream;
//
//
//
//
//     let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
//     let _ = stream.write_all(response).await;
// }

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let listener = Arc::new(listener);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        let listener = Arc::clone(&listener);
        // task::spawn(async move {
        //     handle_client(stream).await;
        // });

        async_std::task::spawn(async move {
            if let Err(err) = handle_connection(stream, client_addr).await {
                eprintln!("Error handling connection: {}", err);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, client_addr: std::net::SocketAddr) -> io::Result<()> {
    // 处理连接逻辑
    println!("Received connection from: {}", client_addr);

    // 读取数据
    let mut buffer = vec![0; 1024];
    let nbytes = match stream.read(&mut buffer).await {
        Ok(n) => n,
        Err(err) => {
            eprintln!("Error reading from stream: {}", err);
            return Err(err.into());
        }
    };

    // 将字节数据转换为字符串
    let received_data = String::from_utf8_lossy(&buffer[..nbytes]);
    println!("Received data from {}: {}", client_addr, received_data);

    // 在这里你可以进一步解析数据，例如将其转换为结构体或其他类型的参数
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
    let _ = stream.write_all(response).await;

    Ok(())
}
