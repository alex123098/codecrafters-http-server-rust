mod request;
mod response;
pub mod server;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum RequestMethod {
    GET,
    POST,
    Unidentified,
}

impl From<&str> for RequestMethod {
    fn from(s: &str) -> Self {
        match s {
            "GET" => RequestMethod::GET,
            "POST" => RequestMethod::POST,
            _ => RequestMethod::Unidentified,
        }
    }
}
