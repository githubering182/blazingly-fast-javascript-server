use super::{Error, MiddlewareHandler, Request};

pub struct Middleware {
    pub handler: Option<MiddlewareHandler>,
    pub next: Option<Box<Middleware>>,
}

impl Default for Middleware {
    fn default() -> Self {
        Self::new()
    }
}

impl Middleware {
    pub fn new() -> Self {
        Self {
            handler: None,
            next: None,
        }
    }

    pub fn add_middleware<F>(&mut self, f: F)
    where
        F: Fn(&Request) -> Result<(), Error> + Sync + Send + 'static,
    {
        if self.handler.is_some() {
            let last = Self::get_last(self);
            last.next = Some(Box::new(Self {
                handler: Some(Box::new(f)),
                next: None,
            }));
            return;
        }
        self.handler = Some(Box::new(f));
    }

    fn get_last(current: &mut Self) -> &mut Self {
        match current.next {
            Some(ref mut next) => Self::get_last(next),
            None => current,
        }
    }

    pub fn handle(&self, request: &mut Request) -> Result<(), Error> {
        if let Some(ref handler) = self.handler {
            handler(request)?;
        }

        if let Some(ref next) = self.next {
            next.handle(request)?;
        }

        Ok(())
    }
}
