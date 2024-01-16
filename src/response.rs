use crate::request::{HTTPRequest, HTTPVersion};
use anyhow::{anyhow, Error, Result};

#[derive(PartialEq, Debug)]
pub enum StatusCode {
    OK,
    Created,
    NotFound,
    BadRequest,
    InternalServerError,
    Unidentified,
}

impl From<&str> for StatusCode {
    fn from(s: &str) -> Self {
        match s {
            "200" => StatusCode::OK,
            "201" => StatusCode::Created,
            "404" => StatusCode::NotFound,
            "400" => StatusCode::BadRequest,
            "500" => StatusCode::InternalServerError,
            _ => StatusCode::Unidentified,
        }
    }
}

impl TryInto<&str> for StatusCode {
    type Error = Error;
    fn try_into(self) -> Result<&'static str> {
        match self {
            StatusCode::OK => Ok("200 OK"),
            StatusCode::Created => Ok("201 Created"),
            StatusCode::NotFound => Ok("404 Not Found"),
            StatusCode::BadRequest => Ok("400 Bad Request"),
            StatusCode::InternalServerError => Ok("500 Internal Server Error"),
            StatusCode::Unidentified => Err(anyhow!("Unknown status code")),
        }
    }
}

#[derive(Debug)]
pub struct ResponseHeader {
    name: String,
    value: String,
}

impl ResponseHeader {
    fn new(name: String, value: String) -> Self {
        ResponseHeader { name, value }
    }
}

#[derive(Debug)]
pub struct HTTPResponse {
    version: HTTPVersion,
    status: StatusCode,
    headers: Vec<ResponseHeader>,
    content: Option<String>,
}

impl HTTPResponse {
    pub fn on_request(request: &HTTPRequest) -> Self {
        HTTPResponse {
            version: request.version(),
            status: StatusCode::Unidentified,
            headers: Vec::new(),
            content: None,
        }
    }

    fn create_headers_section(&self) -> String {
        let mut headers = String::new();
        for header in self.headers.iter() {
            headers.push_str(&format!("{}: {}\r\n", header.name, header.value));
        }
        headers
    }

    pub fn set_status(mut self, status: StatusCode) -> HTTPResponse {
        self.status = status;
        self
    }

    pub fn set_body(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn add_header(mut self, name: &str, value: &str) -> Self {
        self.headers
            .push(ResponseHeader::new(name.to_owned(), value.to_owned()));
        self
    }
}

impl TryInto<String> for HTTPResponse {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        let headers_str = self.create_headers_section();
        let version: &str = self.version.try_into()?;
        let status: &str = self.status.try_into()?;
        let mut result = String::new();
        result.push_str(&format!("{} {}\r\n", version, status));
        if headers_str.len() > 0 {
            result.push_str(&headers_str);
            let length = self.content.as_ref().map_or(0, |c| c.len());
            result.push_str(&format!("Content-Length: {}\r\n", length));
        }
        result.push_str("\r\n");
        if let Some(content) = self.content {
            result.push_str(content.as_str());
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_response_to_string() {
        let response = HTTPResponse {
            version: HTTPVersion::V1_1,
            status: StatusCode::OK,
            headers: Vec::new(),
            content: None,
        };
        let response_str: String = response.try_into().unwrap();

        assert_eq!(response_str, "HTTP/1.1 200 OK\r\n\r\n");
    }

    #[test]
    fn http_response_to_string_with_headers() {
        let response = HTTPResponse {
            version: HTTPVersion::V1_1,
            status: StatusCode::OK,
            headers: vec![ResponseHeader::new(
                "Content-Type".to_string(),
                "text/html".to_string(),
            )],
            content: None,
        };
        let response_str: String = response.try_into().unwrap();

        assert_eq!(
            response_str,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 0\r\n\r\n"
        );
    }

    #[test]
    fn http_response_to_string_with_content() {
        let response = HTTPResponse {
            version: HTTPVersion::V1_1,
            status: StatusCode::OK,
            headers: vec![ResponseHeader::new(
                "Content-Type".to_string(),
                "text/plain".to_string(),
            )],
            content: Some("Hello, world!".to_string()),
        };
        let response_str: String = response.try_into().unwrap();

        assert_eq!(response_str, "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, world!");
    }
}
