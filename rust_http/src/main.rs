mod reader;
mod utils;
use crate::utils::print_request;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                print_request(s);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("client disconnected, {}", e);
                continue;
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }
}

fn get_headers(reader: &mut impl BufRead) -> HashMap<String, String> {
    let mut headers_map = HashMap::new();
    let mut line = String::new();

    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();
        if line == "\r\n" {
            break; // blank line = end of headers
        }
        let parts: Vec<&str> = line.trim_end().splitn(2, ": ").collect();
        if parts.len() == 2 {
            headers_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    headers_map
}

fn get_payload(reader: &mut impl BufRead, content_length: u32) -> String {
    let mut buf = vec![0u8; content_length as usize];

    reader.read_exact(&mut buf).unwrap();

    String::from_utf8_lossy(&buf).to_string()
}
