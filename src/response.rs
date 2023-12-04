use std::collections::HashMap;

pub struct Response {
    pub http: &'static str,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            http: "HTTP/1.1",
            status: 200,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }

    pub fn get_response(mut self) -> String {
        let body = self.body.take().unwrap_or_else(|| "".to_string());

        assert_eq!(
            self.headers
                .entry("Content-length".to_string())
                .or_insert(body.len().to_string()),
            body.len().to_string().as_mut()
        );

        let headers = self.get_headers();

        format!("{} {}\r\n{}\r\n{}", self.http, self.status, headers, body)
    }

    fn get_headers(&mut self) -> String {
        self.headers.iter().fold(String::new(), |prev, (key, val)| {
            prev + &format!("{}: {}\r\n", key, val)
        })
    }

    pub fn set_header<T: ToString>(&mut self, key: &str, value: T) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}
