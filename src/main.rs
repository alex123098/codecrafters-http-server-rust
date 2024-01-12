use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = Vec::new();
                stream.read(&mut buf).unwrap();
                println!("read: {}", String::from_utf8_lossy(&buf));
                stream
                    .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                    .unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
