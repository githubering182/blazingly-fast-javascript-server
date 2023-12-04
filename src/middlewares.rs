use crate::{request::Request, response::Response, MiddlewareHandler};

pub struct Middleware {
    pub handler: Option<MiddlewareHandler>,
    pub next: Option<Box<Middleware>>,
}

impl Middleware {
    pub fn new() -> Self {
        Middleware {
            handler: None,
            next: None,
        }
    }

    pub fn add_middleware<F>(&mut self, f: F)
    where
        F: Fn(&Request, &Response) + Send + 'static,
    {
        if let Some(_) = self.handler {
            let last = Middleware::get_last(self);
            last.next = Some(Box::new(Middleware {
                handler: Some(Box::new(f)),
                next: None,
            }));
            return;
        }
        self.handler = Some(Box::new(f));
    }

    fn get_last(current: &mut Middleware) -> &mut Middleware {
        match current.next {
            Some(ref mut next) => Middleware::get_last(next),
            None => current,
        }
    }

    pub fn handle(&self, request: &mut Request, response: &mut Response) {
        if let Some(ref handler) = self.handler {
            handler(request, response);
        }

        match self.next {
            Some(ref next) => next.handle(request, response),
            None => return,
        }
    }
}
