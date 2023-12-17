mod server;

mod middlewares;
mod request;
mod response;
mod route;
mod workers;

use middlewares::{defaults, middleware::Middleware};
use route::{Method, Route};
use workers::pool::ThreadPool;

pub use request::Request;
pub use response::Response;
pub use server::Server;
