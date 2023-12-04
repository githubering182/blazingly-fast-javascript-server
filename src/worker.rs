use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Error},
    net::TcpStream,
    sync::{mpsc::Receiver, Arc, Mutex, RwLock},
    thread::{spawn, JoinHandle},
};

use crate::{middlewares::Middleware, pages::*, request::Request, Handler};

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Result<TcpStream, Error>>>>,
        routes: Arc<RwLock<HashMap<String, Handler>>>,
        _middleware: &Middleware,
    ) -> Self {
        println!("Worker {id} booting...");

        let mut worker = Self { id, thread: None };

        let thread = spawn(move || loop {
            let stream = receiver.lock().unwrap().recv();

            match stream {
                Ok(stream) => Self::handle_connection(stream, &routes),
                Err(e) => println!("Error getting stream: {:?}", e),
            }
        });

        worker.thread = Some(thread);

        worker
    }

    fn handle_connection(
        stream: Result<TcpStream, Error>,
        routes: &Arc<RwLock<HashMap<String, Handler>>>,
    ) {
        if stream.is_err() {
            println!("Stream handle Error");
            return;
        }

        let mut stream = stream.unwrap();
        let mut request = Request::new();

        Self::parse_stream(&mut request, &stream);

        request.print_request();

        let response = match request.valid {
            false => error(),
            true => match routes.read().unwrap().get(request.route.as_ref().unwrap()) {
                Some(handler) => handler(&request),
                _ => not_found(),
            },
        };

        stream
            .write_all(response.get_response().as_bytes())
            .unwrap();
    }

    fn parse_stream(request: &mut Request, stream: &TcpStream) {
        let buf_reader = BufReader::new(stream);
        request.form_from_reader(buf_reader);
    }
}
