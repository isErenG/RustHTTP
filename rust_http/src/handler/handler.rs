use crate::schemas::{RequestMethod, Response, StatusCode};
use std::collections::HashMap;

pub struct GetHandler;

pub trait RequestHandler {
    fn handle(&self) -> Response;
}

impl RequestHandler for GetHandler {
    fn handle(&self) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Connection".to_string(), "close".to_string());
        Response::new(StatusCode::Ok, headers, Some("Hello\n".to_string()))
    }
}
