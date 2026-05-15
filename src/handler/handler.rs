use std::{pin::Pin, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::schemas::{RequestMethod, Response};

#[derive(Clone)]
pub struct Handler {
    method: RequestMethod,
    path: String,
    handler: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>,
}
impl Handler {
    pub fn create(
        method: RequestMethod,
        path: String,
        handler: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync>,
    ) -> Handler {
        Handler {
            method,
            path,
            handler,
        }
    }

    pub async fn handle(&self, stream: &mut TcpStream) {
        let response = (self.handler)().await;
        stream.write_all(&response.to_bytes()).await.unwrap();
    }

    pub fn get_key(&self) -> String {
        format!("{}:{}", self.method, self.path)
    }
}
