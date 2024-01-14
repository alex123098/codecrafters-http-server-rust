use std::{env, io};

use handlers::FileReader;
use http_server_starter_rust::server::{HTTPResponse, HTTPServer, StatusCode};

mod handlers;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut server = HTTPServer::new(4221);
    let base_dir = get_base_dir().unwrap_or_else(|| "./".to_string());

    server.map_get_fn(
        "/",
        Box::new(|r| HTTPResponse::on_request(&r, StatusCode::OK)),
    );
    server.map_get_fn("/echo/*", Box::new(handlers::handle_echo));
    server.map_get_fn("/user-agent", Box::new(handlers::handle_user_agent));
    let reader = FileReader::new(base_dir);
    server.map_get("/files/*", Box::new(reader));

    server.start().await
}

fn get_base_dir() -> Option<String> {
    let mut args = env::args();
    args.next();
    while let Some(arg) = args.next() {
        if arg == "--directory" {
            return args.next();
        }
    }
    None
}
