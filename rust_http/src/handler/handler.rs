use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::schemas::{RequestMethod, Response};

pub struct Handler {
    method: RequestMethod,
    path: String,
    handler: Box<dyn Fn() -> Response>,
}

impl Handler {
    pub fn create(
        method: RequestMethod,
        path: String,
        handler: Box<dyn Fn() -> Response>,
    ) -> Handler {
        Handler {
            method,
            path,
            handler,
        }
    }

    pub async fn handle(&self, stream: &mut TcpStream) {
        let response = (self.handler)();
        stream.write_all(&response.to_bytes()).await.unwrap();
    }

    pub fn get_key(&self) -> String {
        format!("{}:{}", self.method, self.path)
    }
}
