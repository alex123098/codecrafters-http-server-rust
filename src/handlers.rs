use std::{fs, path::Path};

use http_server_starter_rust::server::{HTTPHandler, HTTPRequest, HTTPResponse, StatusCode};

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

pub struct FileReader {
    base_dir: String,
}

impl FileReader {
    pub fn new(base_dir: String) -> FileReader {
        FileReader { base_dir }
    }
}

impl HTTPHandler for FileReader {
    fn handle(&self, req: &HTTPRequest) -> HTTPResponse {
        let fname = req.path().trim_start_matches("/files/");
        let fpath = Path::new(self.base_dir.as_str()).join(fname);
        if let Ok(content) = fs::read_to_string(&fpath) {
            let mut response = HTTPResponse::on_request(req, StatusCode::OK);
            response.add_header(
                "Content-Type".to_string(),
                "application/octet-stream".to_string(),
            );
            response.set_body(content);
            response
        } else {
            HTTPResponse::on_request(req, StatusCode::NotFound)
        }
    }
}
