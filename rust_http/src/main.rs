pub mod http;
mod reader;
mod schemas;
mod utils;

use std::io;
use std::net::{TcpListener, TcpStream};
use crate::http::parse_http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let request = parse_http(s);
                println!("{}", request.to_string())
            }

            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("client disconnected, {}", e);
                continue;
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }
}
