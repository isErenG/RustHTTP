mod handler;
pub mod http;
mod reader;
mod schemas;
mod server;
mod utils;

use crate::schemas::{RequestMethod, Response, StatusCode};
use crate::server::Server;
use std::collections::HashMap;

fn handle() -> Response {
    let mut headers = HashMap::new();
    headers.insert("Connection".to_string(), "close".to_string());
    Response::new(StatusCode::Ok, headers, Some("Hello\n".to_string()))
}

fn main() {
    let mut server = Server::create_server(7878);
    server.attach_handler(RequestMethod::GET, "/test".to_string(), Box::new(handle));
    server.listen()
}
