use super::Request;
use std::io::Error;

pub mod middleware;

type MiddlewareHandler = Box<dyn Fn(&Request) -> Result<(), Error> + Sync + Send + 'static>;
