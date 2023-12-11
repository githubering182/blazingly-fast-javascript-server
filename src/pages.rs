use crate::response::Response;
use std::io::Error;

pub fn error(error: Option<Error>) -> Response {
    let mut response = Response::new();
    let message = match error {
        Some(e) => e.to_string(),
        _ => "request_error".to_string(),
    };
    response.set_status(400);
    response.set_body(message);
    response
}

pub fn not_found() -> Response {
    let mut response = Response::new();
    response.set_status(404);
    response.set_body("not found".to_string());
    response
}
