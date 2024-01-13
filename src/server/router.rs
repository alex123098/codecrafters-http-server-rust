use crate::request;
use crate::response;
use crate::RequestMethod;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct HandlerDef {
    method: RequestMethod,
    path: String,
    handler: Box<HTTPHandler>,
}

pub type HTTPHandler = fn(&request::HTTPRequest) -> response::HTTPResponse;

#[derive(Clone)]
pub struct Router {
    handlers: Vec<HandlerDef>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            handlers: Vec::new(),
        }
    }

    pub fn add_route(&mut self, method: RequestMethod, path: &str, handler: HTTPHandler) {
        let normalized_path = Self::normalize_path(path);
        self.handlers.push(HandlerDef {
            method,
            path: normalized_path,
            handler: Box::new(handler),
        });
    }

    fn normalize_path(path: &str) -> String {
        let mut normalized = String::new();
        if !path.starts_with('/') {
            normalized.push('/');
        }
        normalized.push_str(path);
        normalized
    }

    pub fn handle_request(&self, request: request::HTTPRequest) -> response::HTTPResponse {
        let handler = self
            .handlers
            .iter()
            .filter(|h| {
                h.method == request.method() && path_matches(request.path(), h.path.as_str())
            })
            .map(|h| h.handler.clone())
            .next();

        if let Some(handler) = handler {
            handler(&request)
        } else {
            response::HTTPResponse::on_request(&request, response::StatusCode::NotFound)
        }
    }
}

fn path_matches(path: &str, pattern: &str) -> bool {
    if !pattern.contains('*') {
        path == pattern
    } else {
        let pattern = &pattern[..pattern.len() - 1];
        path.starts_with(pattern)
    }
}
