use super::{Method, Request};
use std::io::prelude::*;

pub fn form_from_reader(request: &mut Request) {
    let meta = request.raw.by_ref().lines().next();

    let meta: Vec<String> = match meta {
        Some(meta) => meta
            .unwrap()
            .split_whitespace()
            .map(|string| string.to_string())
            .collect(),
        None => {
            return;
        }
    };

    if meta.len() != 3 {
        return;
    }

    request.method = Method::form(meta[0].as_str());
    request.route = Some(meta[1].clone());
    request.http = Some(meta[2].clone());
    request.is_valid = true;
}
