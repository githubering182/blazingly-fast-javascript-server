pub mod middleware;
pub mod request;
pub mod response;
pub mod workers;

pub mod server;

use middleware::middleware::Middleware;
use workers::pool::ThreadPool;

pub use request::request::Request;
pub use response::response::Response;
pub use server::Server;

type Handler = Box<dyn Fn(&Request) -> Response + Sync + Send + 'static>;
