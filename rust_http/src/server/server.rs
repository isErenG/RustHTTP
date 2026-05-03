use crate::handler::RequestHandler;
use crate::http::parse_http;
use crate::schemas::{RequestMethod, Response, StatusCode};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::net::TcpListener;

pub struct Server {
    listener: TcpListener,
    handlers: HashMap<String, Box<dyn RequestHandler>>,
}

impl Server {
    pub fn create_server(port: u32) -> Server {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(addr).unwrap();
        Server {
            listener,
            handlers: HashMap::new(),
        }
    }

    pub fn attach_handler(
        &mut self,
        method: RequestMethod,
        path: String,
        handler: impl RequestHandler + 'static,
    ) {
        let key = format!("{}:{}", method, path);
        self.handlers.insert(key, Box::new(handler));
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let request = parse_http(&mut s);
                    let key = format!("{}:{}", request.request_method, request.path);

                    if !self.handlers.contains_key(&key) {
                        let mut headers = HashMap::new();
                        headers.insert("Connection".to_string(), "close".to_string());
                        let response =
                            Response::new(StatusCode::Ok, headers, Some("Not found\n".to_string()));

                        s.write_all(&response.to_bytes()).unwrap();
                        continue;
                    }

                    let handler = self.handlers.get(&key);

                    let response = handler.unwrap().handle();

                    s.write_all(&response.to_bytes()).unwrap();
                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    println!("client disconnected, {}", e);
                    continue;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            }
        }
    }
}
