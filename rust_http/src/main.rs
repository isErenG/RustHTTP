pub mod http;
mod reader;
mod schemas;
mod utils;

use std::collections::HashMap;
use crate::http::parse_http;
use std::io;
use std::io::Write;
use std::net::TcpListener;
use crate::schemas::{Response, StatusCode};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let request = parse_http(&mut s);
                println!("{}", request.to_string());

                let mut headers = HashMap::new();
                headers.insert("Connection".to_string(), "close".to_string());
                let response = Response::new(StatusCode::Ok, headers, None);

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
