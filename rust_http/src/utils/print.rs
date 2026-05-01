use std::net::TcpStream;
use crate::{get_headers, get_payload};
use crate::reader::CustomReader;

pub fn print_request(s: TcpStream) {
    let mut reader = CustomReader::new(s);
    let headers = get_headers(&mut reader);
    let payload = get_payload(
        &mut reader,
        headers.get("Content-Length").unwrap().parse().unwrap(),
    );

    println!("{:?} {}", headers, payload);
}