use crate::http::parse_http;
use crate::schemas::{RequestMethod, Response, StatusCode};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::pin::Pin;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

pub struct Server {
    listener: TcpListener,
    handlers: HashMap<String, Box<dyn Fn() -> Response>>,
    middleware: Vec<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>>,
}

impl Server {
    pub async fn create_server(port: u32) -> Server {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.unwrap();

        println!("Server created at {}", addr);

        Server {
            listener,
            handlers: HashMap::new(),
            middleware: Vec::new(),
        }
    }

    pub fn attach_middleware(&mut self, m: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>) {
        self.middleware.push(m);
    }

    pub fn attach_handler(
        &mut self,
        method: RequestMethod,
        path: String,
        handler: Box<dyn Fn() -> Response>,
    ) {
        let key = format!("{}:{}", method, path);
        self.handlers.insert(key, handler);
    }

    pub async fn listen(&mut self) {
        loop {
            let (mut s, _) = self.listener.accept().await.unwrap();

            let request = parse_http(&mut s).await;
            let key = format!("{}:{}", request.request_method, request.path);

            if !self.handlers.contains_key(&key) {
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

            let handler = self.handlers.get(&key);

            let response = handler.unwrap()();

            s.write_all(&response.to_bytes()).await.unwrap();
        }
    }
}
