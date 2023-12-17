use super::{Handler, Middleware, Request, Response, ThreadPool};
use std::{
    collections::HashMap,
    io::Error,
    net::TcpListener,
    sync::{Arc, RwLock},
};

pub struct Server {
    address: String,
    routes: Arc<RwLock<HashMap<String, Handler>>>,
    middleware: Arc<RwLock<Middleware>>,
}

impl Server {
    pub fn new(host: &str, port: usize) -> Self {
        Self {
            address: format!("{host}:{port}"),
            routes: Arc::new(RwLock::new(HashMap::new())),
            middleware: Arc::new(RwLock::new(Middleware::new())),
        }
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
    pub fn get<F>(self, route: &str, f: F) -> Self
    where
        F: Fn(&Request) -> Response + Sync + Send + 'static,
    {
        self.routes
            .write()
            .unwrap()
            .insert(route.to_string(), Box::new(f));
        self
    }

    pub fn post<F>(self, route: &str, f: F) -> Self
    where
        F: Fn(&Request) -> Response + Sync + Send + 'static,
    {
        self.routes
            .write()
            .unwrap()
            .insert(route.to_string(), Box::new(f));
        self
    }

    pub fn add_middleware<F>(self, f: F) -> Self
    where
        F: Fn(&Request) -> Result<(), Error> + Sync + Send + 'static,
    {
        self.middleware.write().unwrap().add_middleware(f);
        self
    }
}
