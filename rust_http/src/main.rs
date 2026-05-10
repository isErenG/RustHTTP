mod handler;
pub mod http;
mod middleware;
mod reader;
mod schemas;
mod server;
mod utils;

use crate::handler::Handler;
use crate::schemas::{RequestMethod, Response, StatusCode};
use crate::server::Server;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

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
        Arc::new(|| {
            Box::pin(async {
                let mut headers = HashMap::new();
                headers.insert("Connection".to_string(), "close".to_string());
                tokio::time::sleep(Duration::from_secs(2)).await;
                Response::new(
                    StatusCode::Ok,
                    headers,
                    Some("this is an API!\n".to_string()),
                )
            })
        }),
    );

    server.attach_handler(api_get_handler);
    server.listen().await
}
