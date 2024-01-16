use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use http_server_starter_rust::server::{HTTPHandler, HTTPRequest, HTTPResponse, StatusCode};

pub fn handle_echo(request: &HTTPRequest) -> Result<HTTPResponse> {
    let payload = request.path().trim_start_matches("/echo/");
    let mut response = HTTPResponse::on_request(&request, StatusCode::OK);
    response.add_header("Content-Type", "text/plain");
    response.set_body(payload.to_string());
    Ok(response)
}

pub fn handle_user_agent(request: &HTTPRequest) -> Result<HTTPResponse> {
    let ua = request.header("User-Agent").unwrap_or("");
    let mut response = HTTPResponse::on_request(&request, StatusCode::OK);
    response.add_header("Content-Type", "text/plain");
    response.set_body(ua.to_string());
    Ok(response)
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
    fn handle(&self, req: &HTTPRequest) -> Result<HTTPResponse> {
        let fname = req.path().trim_start_matches("/files/");
        let fpath = Path::new(self.base_dir.as_str()).join(fname);
        if let Ok(content) = fs::read_to_string(&fpath) {
            let mut response = HTTPResponse::on_request(req, StatusCode::OK);
            response.add_header("Content-Type", "application/octet-stream");
            response.set_body(content);
            Ok(response)
        } else {
            Ok(HTTPResponse::on_request(req, StatusCode::NotFound))
        }
    }
}

pub struct FileWriter {
    base_dir: String,
}

impl FileWriter {
    pub fn new(base_dir: String) -> FileWriter {
        FileWriter { base_dir }
    }
}

impl HTTPHandler for FileWriter {
    fn handle(&self, req: &HTTPRequest) -> Result<HTTPResponse> {
        let fname = req.path().trim_start_matches("/files/");
        let fpath = Path::new(self.base_dir.as_str()).join(fname);
        match req.body() {
            Some(body) => {
                let mut file = File::create(&fpath)?;
                file.write_all(body)?;
                file.flush()?;
                Ok(HTTPResponse::on_request(&req, StatusCode::Created))
            }
            None => Ok(HTTPResponse::on_request(&req, StatusCode::BadRequest)),
        }
    }
}
