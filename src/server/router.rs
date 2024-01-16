use anyhow::Result;
use std::sync::{Arc, RwLock};

use crate::{
    server::{HTTPRequest, HTTPResponse, StatusCode},
    RequestMethod,
};

struct Route {
    method: RequestMethod,
    path: String,
    handler: Box<dyn HTTPHandler>,
}

pub trait HTTPHandler: Send + Sync {
    fn handle(&self, req: &HTTPRequest) -> Result<HTTPResponse>;
}

#[derive(Clone)]
pub struct Router {
    handlers: Arc<RwLock<Vec<Route>>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_route(&mut self, method: RequestMethod, path: &str, handler: Box<dyn HTTPHandler>) {
        let normalized_path = Self::normalize_path(path);
        self.handlers.write().unwrap().push(Route {
            method,
            path: normalized_path,
            handler,
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

    pub fn handle_request(&self, request: HTTPRequest) -> HTTPResponse {
        let routes = match self.handlers.read() {
            Ok(r) => r,
            Err(_) => {
                return HTTPResponse::on_request(&request)
                    .set_status(StatusCode::InternalServerError);
            }
        };
        let route = routes
            .iter()
            .filter(|h| {
                h.method == request.method() && path_matches(request.path(), h.path.as_str())
            })
            .next();

        if let Some(route) = route {
            route.handler.handle(&request).unwrap_or_else(|e| {
                HTTPResponse::on_request(&request)
                    .set_status(StatusCode::InternalServerError)
                    .add_header("Content-Type", "text/plain")
                    .set_body(format!("{e}"))
            })
        } else {
            HTTPResponse::on_request(&request).set_status(StatusCode::NotFound)
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
