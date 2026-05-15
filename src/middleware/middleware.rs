use std::pin::Pin;

pub type Middleware = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
