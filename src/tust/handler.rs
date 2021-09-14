use crate::tust::{Request, Response};

pub type PathHandler = fn(&mut Request, &mut Response) -> ();

pub struct Handler {
    pub method: String,
    pub path: String,
    pub handler: PathHandler
}

#[allow(dead_code)]
impl Handler {
    pub fn get(path: &str, handler: PathHandler) -> Self {
        Handler {
            method: "GET".to_owned(),
            path: path.to_owned(),
            handler
        }
    }
    pub fn post(path: &str, handler: PathHandler) -> Self {
        Handler {
            method: "POST".to_owned(),
            path: path.to_owned(),
            handler
        }
    }
    pub fn all(path: &str, handler: PathHandler) -> Self {
        Handler {
            method: "".to_owned(),
            path: path.to_owned(),
            handler
        }
    }
    pub fn other(method: &str, path: &str, handler: PathHandler) -> Self {
        Handler {
            method: method.to_owned(),
            path: path.to_owned(),
            handler
        }
    }
}