use super::{defaults, Method, Middleware, Request, Response, Route, ThreadPool};
use std::{
    collections::HashMap,
    io::Error,
    net::TcpListener,
    sync::{Arc, RwLock},
};

pub struct Server {
    address: String,
    routes: Arc<RwLock<HashMap<String, Route>>>,
    middleware: Arc<RwLock<Middleware>>,
}

impl Server {
    pub fn new(host: &str, port: usize) -> Self {
        let server = Self {
            address: format!("{host}:{port}"),
            routes: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(RwLock::new(Middleware::new())),
        };

        server.add_middleware(|r| {
            defaults::form_from_reader(r);
            Ok(())
        })
    }

    pub fn listen(&self, workers: usize) -> Result<(), Error> {
        let socket = TcpListener::bind(&self.address)?;
        let pool = ThreadPool::new(&self.routes, &self.middleware, workers);

        for stream in socket.incoming() {
            pool.execute(stream);
        }

        Ok(())
    }
}

impl Server {
    pub fn get<F>(self, path: &str, f: F) -> Self
    where
        F: Fn(&Request) -> Response + Sync + Send + 'static,
    {
        let route = Route::new(Method::GET, Box::new(f));
        self.routes.write().unwrap().insert(path.to_string(), route);
        self
    }

    pub fn post<F>(self, path: &str, f: F) -> Self
    where
        F: Fn(&Request) -> Response + Sync + Send + 'static,
    {
        let route = Route::new(Method::POST, Box::new(f));
        self.routes.write().unwrap().insert(path.to_string(), route);
        self
    }

    pub fn add_middleware<F>(self, f: F) -> Self
    where
        F: Fn(&mut Request) -> Result<(), Error> + Sync + Send + 'static,
    {
        self.middleware.write().unwrap().add_middleware(f);
        self
    }
}
