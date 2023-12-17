use super::{Request, Response};

pub type Handler = Box<dyn Fn(&Request) -> Response + Sync + Send + 'static>;

#[derive(PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    Fallback,
}

impl Method {
    pub fn form(method: &str) -> Option<Self> {
        match method {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "PATCH" => Some(Method::PATCH),
            "DELETE" => Some(Method::DELETE),
            _ => None,
        }
    }

    pub fn get(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::Fallback => "no method",
        }
    }
}

pub struct Route {
    pub method: Method,
    pub handler: Handler,
}

impl Route {
    pub fn new<F>(method: Method, handler: Box<F>) -> Self
    where
        F: Fn(&Request) -> Response + Sync + Send + 'static,
    {
        Self { method, handler }
    }
}
