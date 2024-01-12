use crate::request::HTTPVersion;

#[derive(PartialEq, Debug)]
pub enum StatusCode {
    OK,
    NotFound,
    Unidentified,
}

impl From<&str> for StatusCode {
    fn from(s: &str) -> Self {
        match s {
            "200" => StatusCode::OK,
            "404" => StatusCode::NotFound,
            _ => StatusCode::Unidentified,
        }
    }
}

impl TryInto<&str> for StatusCode {
    type Error = &'static str;
    fn try_into(self) -> Result<&'static str, Self::Error> {
        match self {
            StatusCode::OK => Ok("200 OK"),
            StatusCode::NotFound => Ok("404 Not Found"),
            StatusCode::Unidentified => Err("Unknown status code"),
        }
    }
}

#[derive(Debug)]
pub struct HTTPResponse {
    version: HTTPVersion,
    status: StatusCode,
}

impl HTTPResponse {
    pub fn new(version: HTTPVersion, status: StatusCode) -> Self {
        HTTPResponse { version, status }
    }

    pub fn try_to_string(self) -> Result<String, &'static str> {
        let status: &str = self.status.try_into()?;
        let version: &str = self.version.try_into()?;
        Ok(format!("{} {}\r\n\r\n", version, status))
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
        };
        let response_str = response.try_to_string().unwrap();

        assert_eq!(response_str, "HTTP/1.1 200 OK\r\n\r\n");
    }
}
