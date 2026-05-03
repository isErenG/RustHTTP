pub mod http;
mod handler;
mod reader;
mod schemas;
mod server;
mod utils;

use crate::handler::GetHandler;
use crate::schemas::RequestMethod;
use crate::server::Server;

fn main() {
    let mut server = Server::create_server(7878);
    server.attach_handler(RequestMethod::GET, "/test".to_string(), GetHandler);
    server.listen()
}
