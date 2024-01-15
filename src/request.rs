use anyhow::{Error, Result};
use std::collections::HashMap;

use tokio::io::{AsyncBufReadExt, AsyncReadExt};

use crate::RequestMethod;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum HTTPVersion {
    V1_1,
    Unidentified,
}

impl From<&str> for HTTPVersion {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => HTTPVersion::V1_1,
            _ => HTTPVersion::Unidentified,
        }
    }
}

impl TryInto<&str> for HTTPVersion {
    type Error = &'static str;

    fn try_into(self) -> Result<&'static str, Self::Error> {
        match self {
            HTTPVersion::V1_1 => Ok("HTTP/1.1"),
            HTTPVersion::Unidentified => Err("Unknown HTTP version"),
        }
    }
}

#[derive(Debug)]
pub struct HTTPRequest {
    method: RequestMethod,
    path: String,
    version: HTTPVersion,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HTTPRequest {
    pub async fn parse<TStream: AsyncBufReadExt + Unpin>(stream: &mut TStream) -> Result<Self> {
        let mut buf = String::new();
        stream.read_line(&mut buf).await?;
        let first_line = buf.as_str();
        let mut request_def = first_line.split_whitespace();
        let method = request_def
            .next()
            .ok_or(Error::msg("Unknown HTTP Method"))?
            .into();
        let path = request_def
            .next()
            .ok_or(Error::msg("Request prelude is incomplete: missing path"))?
            .to_string();
        let version = request_def
            .next()
            .ok_or(Error::msg("Unsupported protocol"))?
            .into();
        // parse headers
        let mut headers: HashMap<String, String> = HashMap::new();
        loop {
            buf.clear();
            stream.read_line(&mut buf).await?;
            if buf == "\r\n" || buf.is_empty() {
                break;
            }
            let mut header_pair = buf.splitn(2, ": ");
            if let (Some(name), Some(value)) = (header_pair.next(), header_pair.next()) {
                headers.insert(name.to_string(), value[..value.len() - 2].to_string());
            }
        }

        let mut body: Option<Vec<u8>> = None;
        if let Some(body_len) = headers.get("Content-Length") {
            let length = body_len.parse()?;
            let mut body_buf = vec![0u8; length];
            stream.read_exact(&mut body_buf).await?;
            body = Some(body_buf);
        }

        Ok(HTTPRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> HTTPVersion {
        self.version
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_string()).map(|v| v.as_str())
    }

    pub fn method(&self) -> RequestMethod {
        self.method
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn can_parse_get() {
        let payload = r#"GET /resource.txt HTTP/1.1
Host: localhost:4221
User-Agent: test-ua/1.0.0
"#;

        let mut stream = Cursor::new(payload);
        let req = HTTPRequest::parse(&mut stream).await.unwrap();
        assert_eq!(req.method, RequestMethod::GET);
        assert_eq!(req.path, "/resource.txt");
        assert_eq!(req.version, HTTPVersion::V1_1);
    }

    #[tokio::test]
    async fn can_parse_headers() {
        let payload = "GET /resource.txt HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: test-ua/1.0.0\r\n\r\n";
        let mut stream = Cursor::new(payload);
        let req = HTTPRequest::parse(&mut stream).await.unwrap();

        let host = req.header("Host").unwrap();
        assert_eq!(host, "localhost:4221");
        let ua = req.header("User-Agent").unwrap();
        assert_eq!(ua, "test-ua/1.0.0");
    }

    #[tokio::test]
    async fn can_parse_body() {
        let payload = "POST /resource.txt HTTP/1.1\r\nHost: localhost:4221\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest";
        let mut stream = Cursor::new(payload);
        let req = HTTPRequest::parse(&mut stream).await.unwrap();

        let body = req.body.unwrap();
        assert_eq!(String::from_utf8(body).unwrap(), "test");
    }
}
