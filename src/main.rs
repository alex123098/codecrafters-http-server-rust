use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use response::{HTTPResponse, StatusCode};

use crate::request::HTTPRequest;

mod request;
mod response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("accepted new connection");

    let payload_bytes = read_data(&mut stream).unwrap();
    let payload = String::from_utf8_lossy(&payload_bytes);
    let request = HTTPRequest::parse(&payload).unwrap();
    println!("received request: {:?}", &request);
    let response = handle_request(request);
    println!("sending response: {:?}", &response);

    stream
        .write_all(response.try_to_string().unwrap().as_bytes())
        .unwrap();
}

fn handle_request(request: HTTPRequest) -> HTTPResponse {
    if request.path() == "/" {
        HTTPResponse::on_request(&request, StatusCode::OK)
    } else if request.path().starts_with("/echo/") {
        let echo = request.path().trim_start_matches("/echo/");
        let mut response = HTTPResponse::on_request(&request, StatusCode::OK);
        response.add_header("Content-Type".to_string(), "text/plain".to_string());
        response.set_content(echo.to_string());
        response
    } else {
        HTTPResponse::on_request(&request, StatusCode::NotFound)
    }
}

fn read_data(stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = [0u8; 1024];
    let mut payload = Vec::new();
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        payload.extend_from_slice(&buffer[..bytes_read]);
        if bytes_read < buffer.len() {
            break;
        }
    }
    Ok(payload)
}
