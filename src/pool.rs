use crate::{middlewares::Middleware, worker::Worker, Handler};
use std::{
    collections::HashMap,
    io::Error,
    net::TcpStream,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex, RwLock,
    },
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Result<TcpStream, Error>>>,
}

impl ThreadPool {
    pub fn new(
        routes: &Arc<RwLock<HashMap<String, Handler>>>,
        middleware: &Middleware,
        workers: usize,
    ) -> Self {
        assert!(workers > 0);

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (1..=workers)
            .map(|id| Worker::new(id, Arc::clone(&receiver), Arc::clone(routes), middleware))
            .collect::<Vec<Worker>>();

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute(&self, stream: Result<TcpStream, Error>) {
        self.sender.as_ref().unwrap().send(stream).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting worker {} down", worker.id);

            if let Some(worker) = worker.thread.take() {
                worker.join().unwrap();
            }
        }
    }
}
