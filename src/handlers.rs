use anyhow::Result;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use http_server_starter_rust::server::{HTTPHandler, HTTPRequest, HTTPResponse, StatusCode};

pub fn handle_echo(request: &HTTPRequest) -> Result<HTTPResponse> {
    let payload = request.path().trim_start_matches("/echo/");
    Ok(HTTPResponse::on_request(request)
        .set_status(StatusCode::OK)
        .add_header("Content-Type", "text/plain")
        .set_body(payload.to_string()))
}

pub fn handle_user_agent(request: &HTTPRequest) -> Result<HTTPResponse> {
    let ua = request.header("User-Agent").unwrap_or("");
    Ok(HTTPResponse::on_request(request)
        .set_status(StatusCode::OK)
        .add_header("Content-Type", "text/plain")
        .set_body(ua.to_owned()))
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
        if let Ok(content) = fs::read_to_string(fpath) {
            Ok(HTTPResponse::on_request(req)
                .set_status(StatusCode::OK)
                .add_header("Content-Type", "application/octet-stream")
                .set_body(content))
        } else {
            Ok(HTTPResponse::on_request(req).set_status(StatusCode::NotFound))
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
                let mut file = File::create(fpath)?;
                file.write_all(body)?;
                file.flush()?;
                Ok(HTTPResponse::on_request(req).set_status(StatusCode::Created))
            }
            None => Ok(HTTPResponse::on_request(req).set_status(StatusCode::BadRequest)),
        }
    }
}
