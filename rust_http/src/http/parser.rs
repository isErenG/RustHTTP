use crate::reader::CustomReader;
use crate::schemas::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::net::TcpStream;

pub fn parse_http(s: &mut TcpStream) -> Request {
    let mut reader = CustomReader::new(s);

    let (method, path) = get_request_line(&mut reader);
    let headers = get_headers(&mut reader);

    let mut payload = Option::None;
    if method == RequestMethod::POST {
        payload = Option::from(get_payload(
            &mut reader,
            headers.get("Content-Length").unwrap().parse().unwrap(),
        ));
    }

    Request::new(path, method, headers, payload)
}

fn get_request_line(reader: &mut impl BufRead) -> (RequestMethod, String) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let split_line: Vec<_> = line.splitn(3, " ").collect();
    match split_line[0] {
        "GET" => (RequestMethod::GET, split_line[1].parse().unwrap()),
        "POST" => (RequestMethod::POST, split_line[1].parse().unwrap()),
        _ => panic!("yo tf"),
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
