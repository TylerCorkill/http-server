use std::io::prelude::*;
use std::net::TcpListener;

use crate::tust::{Handler, PathHandler, Request, Response};

pub struct Server {
    handlers: Vec<Handler>,
    handler_lock: bool
}

#[allow(dead_code)]
impl Server {
    pub fn init(start: fn(&mut Server) -> ()) -> Self {
        let mut server = Server { handlers: vec![], handler_lock: false };
        start(&mut server);
        server.handlers.shrink_to_fit();
        return server;
    }
    pub fn listen(&self, port: u16) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut buffer = [0; 1024];
            let mut stream = stream.unwrap();

            stream.read(&mut buffer).unwrap();

            let mut req = Request::new(String::from_utf8_lossy(&buffer[..]).as_ref());
            let mut res = Response::new();

            print!("{} {} ", req.method, req.path);

            for h in &self.handlers {
                if req.matches(h) {
                    (h.handler)(&mut req, &mut res);
                    if res.complete {
                        println!("{} {}", res.status_code, res.status_text);

                        stream.write(res.format().as_bytes()).unwrap();
                        stream.flush().unwrap();

                        break;
                    }
                }
            }

            // Debug
            // println!("{}", req.method);
            // println!("{}", req.path);
            // println!("{}", req._headers);
            // for (h, v) in req._header_map.iter() {
            //     println!("{}: {}", h, v);
            // }
            // println!("{}", req.header("Content-Type").unwrap());
            // println!("{}", req.data);
        }
    }
    fn add_handler(&mut self, handler: Handler) {
        if self.handler_lock {
            println!("Cannot add handler after server initialization")
        } else {
            self.handlers.push(handler)
        }
    }
    pub fn get(&mut self, path: &str, handler: PathHandler) -> () {
        self.add_handler(Handler {
            method: "GET".to_owned(),
            path: path.to_owned(),
            handler
        })

    }
    pub fn post(&mut self, path: &str, handler: PathHandler) -> () {
        self.add_handler(Handler {
            method: "POST".to_owned(),
            path: path.to_owned(),
            handler
        })
    }
    pub fn all(&mut self, path: &str, handler: PathHandler) -> () {
        self.add_handler(Handler {
            method: "".to_owned(),
            path: path.to_owned(),
            handler
        })
    }
    pub fn other(&mut self, method: &str, path: &str, handler: PathHandler) -> () {
        self.add_handler(Handler {
            method: method.to_owned(),
            path: path.to_owned(),
            handler
        })
    }


}