use crate::reader::CustomAsyncReader;
use crate::schemas::*;
use std::collections::HashMap;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use tokio::{io::AsyncBufRead, net::TcpStream};

pub async fn parse_http(s: &mut TcpStream) -> Request {
    let mut reader = CustomAsyncReader::new(s);

    let (method, path) = get_request_line(&mut reader).await;
    let headers = get_headers(&mut reader).await;

    let mut payload = Option::None;
    if method == RequestMethod::POST {
        if let Some(len) = headers.get("Content-Length") {
            payload = Option::from(get_payload(&mut reader, len.parse().unwrap()).await);
        }
    }

    Request::new(path, method, headers, payload)
}

async fn get_request_line(reader: &mut (impl AsyncBufRead + Unpin)) -> (RequestMethod, String) {
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();

    let split_line: Vec<_> = line.splitn(3, " ").collect();
    match split_line[0] {
        "GET" => (RequestMethod::GET, split_line[1].parse().unwrap()),
        "POST" => (RequestMethod::POST, split_line[1].parse().unwrap()),
        _ => {
            eprintln!("Unknown method: {:?}", split_line[0]);
            panic!("Fatal horrible crash");
        }
    }
}

async fn get_headers(reader: &mut (impl AsyncBufRead + Unpin)) -> HashMap<String, String> {
    let mut headers_map = HashMap::new();
    let mut line = String::new();

    loop {
        line.clear();
        reader.read_line(&mut line).await.unwrap();
        if line == "\r\n" {
            break; // blank line = end of headers
        }
        let parts: Vec<&str> = line.trim_end().splitn(2, ": ").collect();
        if parts.len() == 2 {
            headers_map.insert(parts[0].to_lowercase(), parts[1].to_string());
        }
    }

    headers_map
}

async fn get_payload(reader: &mut (impl AsyncBufRead + Unpin), content_length: u32) -> String {
    let mut buf = vec![0u8; content_length as usize];

    reader.read_exact(&mut buf).await.unwrap();

    String::from_utf8_lossy(&buf).to_string()
}
