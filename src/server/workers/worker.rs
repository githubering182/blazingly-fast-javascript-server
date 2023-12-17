use super::{fallback_handlers::*, Middleware, Request, Route};
use std::{
    collections::HashMap,
    io::{prelude::*, Error},
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
        routes: Arc<RwLock<HashMap<String, Route>>>,
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
        routes: &Arc<RwLock<HashMap<String, Route>>>,
        middleware: &Arc<RwLock<Middleware>>,
    ) {
        if stream.is_err() {
            println!("Stream handle Error");
            return;
        }

        let mut stream = stream.unwrap();
        let mut request = Request::new(&stream);

        if middleware.read().unwrap().handle(&mut request).is_ok() {
            request.is_valid = true;
        };

        request.print_request();

        let request_route = match request.route {
            Some(ref route) => route,
            None => "",
        };

        let response = match routes.read().unwrap().get(request_route) {
            _ if !request.is_valid => error(None),
            Some(route_handler) if Worker::check_method(&request, route_handler) => {
                let handler = route_handler.handler.as_ref();
                handler(&request)
            }
            _ => not_found(),
        };

        stream
            .write_all(response.get_response().as_bytes())
            .unwrap();
    }

    fn check_method(request: &Request, handler: &Route) -> bool {
        request.method.as_ref().unwrap() == &handler.method
    }
}
