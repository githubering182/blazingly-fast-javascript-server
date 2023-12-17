use super::{Method, Request};
use std::io::Error;

pub mod defaults;
pub mod middleware;

type MiddlewareHandler = Box<dyn Fn(&mut Request) -> Result<(), Error> + Sync + Send + 'static>;
