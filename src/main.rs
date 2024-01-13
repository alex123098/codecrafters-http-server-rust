use std::io;

use http_server_starter_rust::server::{HTTPResponse, HTTPServer, StatusCode};

mod handlers;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut server = HTTPServer::new(4221);

    server.map_get("/", |r| HTTPResponse::on_request(&r, StatusCode::OK));
    server.map_get("/echo/*", handlers::handle_echo);
    server.map_get("/user-agent", handlers::handle_user_agent);

    server.start().await
}
