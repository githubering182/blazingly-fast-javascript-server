use super::{pages::*, Handler, Middleware, Request, Response};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Error},
    net::TcpStream,
    sync::{mpsc::Receiver, Arc, Mutex, RwLock},
    thread::{spawn, JoinHandle},
};

pub struct Worker {
    pub id: usize,
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Result<TcpStream, Error>>>>,
        routes: Arc<RwLock<HashMap<String, Handler>>>,
        middleware: Arc<RwLock<Middleware>>,
    ) -> Self {
        println!("Worker {id} booting...");

        let mut worker = Self { id, thread: None };

        let thread = spawn(move || loop {
            let stream = receiver.lock().unwrap().recv();

            match stream {
                Ok(stream) => Self::handle_connection(stream, &routes, &middleware),
                Err(e) => {
                    println!("Error getting stream: {:?}", e);
                    break;
                }
            }
        });

        worker.thread = Some(thread);

        worker
    }

    fn handle_connection(
        stream: Result<TcpStream, Error>,
        routes: &Arc<RwLock<HashMap<String, Handler>>>,
        middleware: &Arc<RwLock<Middleware>>,
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
            false => error(None),
            true => Self::proceed_request(request, routes, middleware),
        };

        stream
            .write_all(response.get_response().as_bytes())
            .unwrap();
    }

    fn proceed_request(
        mut request: Request,
        routes: &Arc<RwLock<HashMap<String, Handler>>>,
        middleware: &Arc<RwLock<Middleware>>,
    ) -> Response {
        match middleware.read().unwrap().handle(&mut request) {
            Err(e) => {
                return error(Some(e));
            }
            Ok(_) => (),
        }

        match routes.read().unwrap().get(request.route.as_ref().unwrap()) {
            Some(handler) => handler(&request),
            _ => not_found(),
        }
    }

    fn parse_stream(request: &mut Request, stream: &TcpStream) {
        let buf_reader = BufReader::new(stream);
        request.form_from_reader(buf_reader);
    }
}
