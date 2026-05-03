use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;
use std::io::Read;
use std::ops::{Deref, DerefMut};

pub struct Request {
    pub path: String,
    pub request_method: RequestMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(
        path: String,
        request_method: RequestMethod,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Request {
        Request {
            path,
            request_method,
            headers,
            body,
        }
    }
}
#[derive(PartialEq, Eq)]
pub enum RequestMethod {
    GET,
    POST,
}

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

pub enum StatusCode {
    Ok,         // 200
    NotFound,   // 404
    BadRequest, // 400
}

impl Response {
    pub fn new(
        status: StatusCode,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Response {
        Response {
            status,
            headers,
            body,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let status_line = match self.status {
            StatusCode::Ok => "HTTP/1.1 200 OK",
            StatusCode::NotFound => "HTTP/1.1 404 Not Found",
            StatusCode::BadRequest => "HTTP/1.1 400 Bad Request",
        };

        let mut response = String::new();
        response.push_str(status_line);
        response.push_str("\r\n");

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        if let Some(body) = &self.body {
            response.push_str(body);
        }

        response.into_bytes()
    }
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestMethod::GET => write!(f, "GET"),
            RequestMethod::POST => write!(f, "POST"),
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", self.request_method, self.path)?;
        for (key, value) in &self.headers {
            writeln!(f, "  {}: {}", key, value)?;
        }

        if let Some(body) = &self.body {
            if body.is_empty() {
                return Err(Error);
            }
            writeln!(f, "  Body: {}", body.to_string())?;
        }

        Ok(())
    }
}
