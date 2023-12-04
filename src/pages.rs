use crate::response::Response;

pub fn error() -> Response {
    let mut response = Response::new();
    response.set_status(400);
    response.set_body("request error".to_string());
    response
}

pub fn not_found() -> Response {
    let mut response = Response::new();
    response.set_status(404);
    response.set_body("not found".to_string());
    response
}
