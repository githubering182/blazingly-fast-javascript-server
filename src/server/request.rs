use super::Method;
use std::{io::BufReader, net::TcpStream};

pub struct Request<'a> {
    pub raw: BufReader<&'a TcpStream>,
    pub method: Option<Method>,
    pub route: Option<String>,
    pub http: Option<String>,
    pub is_valid: bool,
}

impl<'a> Request<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);

        Self {
            raw: buf_reader,
            method: None,
            route: None,
            http: None,
            is_valid: false,
        }
    }

    pub fn print_request(&self) {
        if !self.is_valid {
            println!("Request is invalid");
            return;
        }
        println!(
            "NEW {}: {}",
            self.method.as_ref().unwrap_or(&Method::Fallback).get(),
            self.route.as_deref().unwrap_or("no route")
        );
    }
}
