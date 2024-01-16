use anyhow::{Error, Result};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

use crate::RequestMethod;
pub use crate::{
    request::HTTPRequest,
    response::{HTTPResponse, StatusCode},
};
pub use router::HTTPHandler;

mod router;

struct FuncHTTPHandler {
    handler_fn: Box<dyn Fn(&HTTPRequest) -> Result<HTTPResponse> + Send + Sync + 'static>,
}

impl HTTPHandler for FuncHTTPHandler {
    fn handle(&self, req: &HTTPRequest) -> Result<HTTPResponse> {
        (self.handler_fn)(req)
    }
}

pub struct HTTPServer {
    port: u16,
    router: router::Router,
}

impl HTTPServer {
    pub fn new(port: u16) -> HTTPServer {
        HTTPServer {
            port: port,
            router: router::Router::new(),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).await?;

        println!("Serving on port {}", self.port);

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let router = self.router.clone();
                    tokio::spawn(async move {
                        serve(router, stream).await.unwrap();
                    });
                }
                Err(e) => {
                    println!("Unable to accept client connection: {}", e);
                }
            }
        }
    }

    pub fn map_get_fn(
        &mut self,
        path: &str,
        handler: Box<fn(&HTTPRequest) -> Result<HTTPResponse>>,
    ) {
        self.router.add_route(
            RequestMethod::GET,
            path,
            Box::new(FuncHTTPHandler {
                handler_fn: handler,
            }),
        );
    }

    pub fn map_get(&mut self, path: &str, handler: Box<dyn HTTPHandler>) {
        self.router.add_route(RequestMethod::GET, path, handler);
    }

    pub fn map_post(&mut self, path: &str, handler: Box<dyn HTTPHandler>) {
        self.router.add_route(RequestMethod::POST, path, handler);
    }
}

async fn serve<TStream: AsyncReadExt + AsyncWriteExt + Unpin>(
    router: router::Router,
    mut stream: TStream,
) -> Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let request = HTTPRequest::parse(&mut reader).await?;
    dbg!(&request);
    let response = router.handle_request(request);
    write_response(&mut stream, response).await?;
    Ok(())
}

async fn write_response<TStream: AsyncWriteExt + Unpin>(
    stream: &mut TStream,
    response: HTTPResponse,
) -> Result<()> {
    if let Ok(response_str) = TryInto::<String>::try_into(response) {
        stream.write_all(response_str.as_bytes()).await?;
        Ok(())
    } else {
        Err(Error::msg("Unable to serialize response"))
    }
}
