use crate::handler::Handler;
use crate::http::parse_http;
use crate::schemas::{Response, StatusCode};
use std::{collections::HashMap, pin::Pin};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    handlers: Vec<Handler>,
    middleware: Vec<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>>,
}

impl Server {
    pub async fn create_server(port: u32) -> Server {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        println!("Server created at {}", addr);

        Server {
            listener,
            handlers: Vec::new(),
            middleware: Vec::new(),
        }
    }

    pub fn attach_middleware(&mut self, m: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>) {
        self.middleware.push(m);
    }

    pub fn attach_handler(&mut self, handler: Handler) {
        self.handlers.push(handler);
    }

    pub async fn listen(&mut self) {
        loop {
            let (mut s, _) = self.listener.accept().await.unwrap();

            let request = parse_http(&mut s).await;
            let key = format!("{}:{}", request.request_method, request.path);

            let mut method_handle: Option<&Handler> = None;
            for handler in &self.handlers {
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
                continue;
            }

            for middleware in &self.middleware {
                middleware().await;
            }

            method_handle.unwrap().handle(&mut s).await;
        }
    }
}
