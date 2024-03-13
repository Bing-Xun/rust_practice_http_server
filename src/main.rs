use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 创建一个原子整数，并初始化为 0
    let atomic_int = Arc::new(AtomicI32::new(0));


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let atomic_int_clone = Arc::clone(&atomic_int);

        thread::spawn(move || {
            handle_connection(stream);

            let atomic_int_inner = &atomic_int_clone;
            atomic_int_inner.fetch_add(1, Ordering::SeqCst);

            let value = atomic_int_inner.load(Ordering::SeqCst);
            println!("Result of atomic operation: {}", value);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("請求：{:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();

    // shared_number.fetch_add(1, Ordering::SeqCst);
    // let value = shared_number.load(Ordering::SeqCst);
}