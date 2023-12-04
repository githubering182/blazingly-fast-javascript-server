use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub struct Request {
    pub method: Option<String>,
    pub route: Option<String>,
    pub http: Option<String>,
    pub valid: bool,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}
impl Request {
    pub fn new() -> Self {
        Self {
            method: None,
            route: None,
            http: None,
            valid: false,
        }
    }

    pub fn form_from_reader(&mut self, buf: BufReader<&TcpStream>) {
        let meta = buf.lines().next();

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

        self.method = Some(meta[0].clone());
        self.route = Some(meta[1].clone());
        self.http = Some(meta[2].clone());
        self.valid = true;
    }

    pub fn print_request(&self) {
        if !self.valid {
            println!("Request is invalid");
            return;
        }
        println!(
            "NEW {}: {}",
            self.method.as_deref().unwrap_or("no method"),
            self.route.as_deref().unwrap_or("no route")
        );
    }
}
