use std::collections::HashMap;
use std::io;
use std::io::Read;
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

fn print_request(mut s: TcpStream) {
    let headers = get_headers(&mut s);
    let payload = get_payload(
        &mut s,
        headers.get("Content-Length").unwrap().parse().unwrap(),
    );

    println!("{:?} {}", headers, payload);
}

fn get_headers(s: &mut TcpStream) -> HashMap<String, String> {
    let mut headers_map = HashMap::new();

    let mut buf = [0; 10];
    let mut chunks: Vec<u8> = Vec::new();

    loop {
        let bytes_read = s.read(&mut buf).unwrap();
        chunks.extend_from_slice(&buf[0..bytes_read]);

        if chunks.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    let header_str = String::from_utf8_lossy(&chunks).to_string();

    header_str.split("\r\n").for_each(|header| {
        let parts: Vec<&str> = header.splitn(2, ": ").collect();

        if parts.len() == 2 {
            headers_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    });

    return headers_map;
}

fn get_payload(s: &mut TcpStream, content_length: u32) -> String {
    let payload: String;

    let mut buf = vec![0u8; content_length as usize];

    s.read_exact(&mut buf).unwrap();

    payload = String::from_utf8_lossy(&buf).to_string();

    return payload;
}
