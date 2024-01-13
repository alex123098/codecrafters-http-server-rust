use http_server_starter_rust::server::{HTTPRequest, HTTPResponse, StatusCode};

pub fn handle_echo(request: &HTTPRequest) -> HTTPResponse {
    let payload = request.path().trim_start_matches("/echo/");
    let mut response = HTTPResponse::on_request(&request, StatusCode::OK);
    response.set_body(payload.to_string());
    response
}

pub fn handle_user_agent(request: &HTTPRequest) -> HTTPResponse {
    let ua = request.header("User-Agent").unwrap_or("");
    let mut response = HTTPResponse::on_request(&request, StatusCode::OK);
    response.add_header("Content-Type".to_string(), "text/plain".to_string());
    response.set_body(ua.to_string());
    response
}
