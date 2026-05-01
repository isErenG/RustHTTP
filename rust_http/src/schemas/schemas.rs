use std::collections::HashMap;
use std::fmt;

pub struct Request {
    path: String,
    request_method: RequestMethod,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn new(
        path: String,
        request_method: RequestMethod,
        headers: HashMap<String, String>,
        body: String,
    ) -> Request {
        Request {
            path,
            request_method,
            headers,
            body,
        }
    }
}

pub enum RequestMethod {
    GET,
    POST,
}

struct Response {}

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
        if !self.body.is_empty() {
            writeln!(f, "  Body: {}", self.body)?;
        }
        Ok(())
    }
}
