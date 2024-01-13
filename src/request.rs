use std::collections::HashMap;

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
}

impl HTTPRequest {
    pub fn parse(buf: &str) -> Option<Self> {
        let mut lines = buf.lines();
        let first_line = lines.next()?;
        let mut request_def = first_line.split_whitespace();
        let method = request_def.next()?.into();
        let path = request_def.next()?.to_string();
        let version = request_def.next()?.into();
        let mut headers = HashMap::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut header_pair = line.splitn(2, ": ");
            if let (Some(name), Some(value)) = (header_pair.next(), header_pair.next()) {
                headers.insert(name.to_string(), value.to_string());
            }
        }

        Some(HTTPRequest {
            method,
            path,
            version,
            headers,
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
    use super::*;

    #[test]
    fn can_parse_get() {
        let payload = r#"GET /resource.txt HTTP/1.1
Host: localhost:4221
User-Agent: test-ua/1.0.0
"#;
        let req = HTTPRequest::parse(payload).unwrap();
        assert_eq!(req.method, RequestMethod::GET);
        assert_eq!(req.path, "/resource.txt");
        assert_eq!(req.version, HTTPVersion::V1_1);
    }

    #[test]
    fn can_parse_headers() {
        let payload = "GET /resource.txt HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: test-ua/1.0.0\r\n\r\n";
        let req = HTTPRequest::parse(payload).unwrap();

        let host = req.header("Host").unwrap();
        assert_eq!(host, "localhost:4221");
        let ua = req.header("User-Agent").unwrap();
        assert_eq!(ua, "test-ua/1.0.0");
    }
}
