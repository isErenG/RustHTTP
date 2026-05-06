mod handler;
pub mod http;
mod reader;
mod schemas;
mod server;
mod utils;

use crate::handler::Handler;
use crate::schemas::{RequestMethod, Response, StatusCode};
use crate::server::Server;
use std::collections::HashMap;

fn handle() -> Response {
    let mut headers = HashMap::new();
    headers.insert("Connection".to_string(), "close".to_string());
    Response::new(StatusCode::Ok, headers, Some("Hello\n".to_string()))
}
#[tokio::main]
async fn main() {
    let mut server = Server::create_server(7878).await;

    server.attach_middleware(Box::new(|| {
        Box::pin(async {
            println!("Middleware running!");
        })
    }));

    let api_get_handler = Handler::create(
        RequestMethod::GET,
        "/api".to_string(),
        Box::new(|| {
            let mut headers = HashMap::new();
            headers.insert("Connection".to_string(), "close".to_string());
            Response::new(
                StatusCode::Ok,
                headers,
                Some("this is an API!\n".to_string()),
            )
        }),
    );

    server.attach_handler(api_get_handler);
    server.listen().await
}
