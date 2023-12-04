pub mod middlewares;
pub mod pages;
pub mod pool;
pub mod request;
pub mod response;
pub mod server;
pub mod worker;

type Handler = Box<dyn Fn(&request::Request) -> response::Response + Sync + Send + 'static>;
type MiddlewareHandler = Box<dyn Fn(&request::Request, &response::Response) + Send + 'static>;
