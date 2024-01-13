#[derive(PartialEq, Debug)]
pub enum RequestMethod {
    GET,
    Unidentified,
}

impl From<&str> for RequestMethod {
    fn from(s: &str) -> Self {
        match s {
            "GET" => RequestMethod::GET,
            _ => RequestMethod::Unidentified,
        }
    }
}

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
}

impl HTTPRequest {
    pub fn parse(buf: &str) -> Option<Self> {
        let mut lines = buf.lines();
        let first_line = lines.next()?;
        let mut request_def = first_line.split_whitespace();
        let method = request_def.next()?.into();
        let path = request_def.next()?.to_string();
        let version = request_def.next()?.into();
        Some(HTTPRequest {
            method,
            path,
            version,
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> HTTPVersion {
        self.version
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
}
