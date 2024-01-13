use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub use crate::request::HTTPRequest;
pub use crate::response::{HTTPResponse, StatusCode};
use crate::RequestMethod;

mod router;

pub use router::HTTPHandler;

pub struct HTTPServer {
    port: u16,
    router: router::Router,
}

impl HTTPServer {
    pub fn new(port: u16) -> HTTPServer {
        HTTPServer {
            port: port,
            router: router::Router::new(),
        }
    }

    pub async fn start(&self) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).await?;

        println!("Serving on port {}", self.port);

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let router = self.router.clone();
                    tokio::spawn(async move {
                        serve(router, stream).await.unwrap();
                    });
                }
                Err(e) => {
                    println!("Unable to accept client connection: {}", e);
                }
            }
        }
    }

    pub fn map_get(&mut self, path: &str, handler: HTTPHandler) {
        self.router.add_route(RequestMethod::GET, path, handler);
    }

    pub fn map_post(&mut self, path: &str, handler: HTTPHandler) {
        self.router.add_route(RequestMethod::POST, path, handler);
    }
}

async fn serve(router: router::Router, mut stream: TcpStream) -> io::Result<()> {
    let req_bytes = read_request(&mut stream).await?;
    let req_str = String::from_utf8_lossy(&req_bytes);
    if let Some(request) = HTTPRequest::parse(&req_str) {
        let response = router.handle_request(request);
        write_response(&mut stream, response).await?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to parse request",
        ))
    }
}

async fn read_request(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut buffer = [0u8; 1024];
    let mut payload = Vec::new();
    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        payload.extend_from_slice(&buffer[..bytes_read]);
        if bytes_read < buffer.len() {
            break;
        }
    }
    Ok(payload)
}

async fn write_response(stream: &mut TcpStream, response: HTTPResponse) -> io::Result<()> {
    if let Ok(response_str) = response.try_to_string() {
        stream.write_all(response_str.as_bytes()).await?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to serialize response",
        ))
    }
}
