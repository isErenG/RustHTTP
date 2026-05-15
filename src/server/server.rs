use crate::handler::Handler;
use crate::http::parse_http;
use crate::middleware::{self, Middleware};
use crate::schemas::{Response, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    handlers: Arc<Vec<Handler>>,
    middleware: Vec<Middleware>,
}

impl Server {
    pub async fn create_server(port: u32) -> Server {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        println!("Server created at {}", addr);

        Server {
            listener,
            handlers: Arc::new(Vec::new()),
            middleware: Vec::new(),
        }
    }

    pub fn attach_middleware(&mut self, m: Middleware) {
        self.middleware.push(m);
    }

    pub fn attach_handler(&mut self, handler: Handler) {
        Arc::make_mut(&mut self.handlers).push(handler);
    }

    pub async fn listen(&mut self) {
        let middlewares = Arc::new(std::mem::take(&mut self.middleware));
        loop {
            let (mut s, _) = self.listener.accept().await.unwrap();
            let handlers = Arc::clone(&self.handlers);
            let middlewares = Arc::clone(&middlewares);
            tokio::spawn(async move {
                let request = parse_http(&mut s).await;
                let key = format!("{}:{}", request.request_method, request.path);

                let mut method_handle: Option<&Handler> = None;
                for handler in handlers.iter() {
                    if handler.get_key() == key {
                        method_handle = Some(handler);
                        break;
                    }
                    method_handle = None;
                }

                if method_handle.is_none() {
                    let mut headers = HashMap::new();
                    headers.insert("Connection".to_string(), "close".to_string());
                    let response =
                        Response::new(StatusCode::Ok, headers, Some("Not found\n".to_string()));

                    s.write_all(&response.to_bytes()).await.unwrap();
                    return;
                }

                for m in middlewares.iter() {
                    m().await;
                }

                method_handle.unwrap().handle(&mut s).await;
            });
        }
    }
}
