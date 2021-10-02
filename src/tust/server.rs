use std::io::prelude::*;
use std::net::TcpListener;

use crate::tust::{HandlerTree, PathHandler, Request, Response};

pub struct Server {
    handler_lock: bool,
    handler_tree: HandlerTree
}

#[allow(dead_code)]
impl Server {
    pub fn init(start: fn(&mut Server) -> ()) -> Self {
        let mut server = Server { handler_lock: false, handler_tree: HandlerTree::new() };
        start(&mut server);
        server.handler_tree.shrink_to_fit();
        server.handler_lock = true;
        println!("Initialization Complete");
        return server;
    }
    pub fn listen(&self, port: u16) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        println!("Listening on port {}", port);
        for stream in listener.incoming() {
            // TODO [1] Buffer Performance
            let mut buffer = [0; 1024];
            let mut stream = stream.unwrap();

            stream.read(&mut buffer).expect("Buffer Read Error");

            let mut end_index = 1023;
            while (end_index != 0) & (buffer[end_index] == 0) { end_index -= 1 }
            end_index += 1;

            let mut req = Request::new(&String::from_utf8_lossy(&buffer[..end_index]));
            let mut res = Response::new();

            self.resolve(&mut req, &mut res);

            if res.complete {
                // Log request and response status
                println!("{} {} {} {}", req.method, req.path, res.status_code, res.status_text);

                // TODO [4] Response formatter to byte slice
                stream.write(format!("{}", res).as_bytes()).expect("Failed to write to stream");
                stream.flush().expect("Failed to flush stream");
            } else {
                println!("{} {} Unhandled Route", req.method, req.path);
            }
        }
    }

    // Handler Tree fns
    fn resolve(&self, req: &mut Request, res: &mut Response) {
        let p = req.path.clone();
        let path: Vec<&str> = p.split("/").skip(1).collect();
        self.handler_tree.resolve(&path, req, res);
    }
    fn add_handler(&mut self, method: &str, path: &str, handler: PathHandler) {
        if self.handler_lock {
            panic!("Cannot add handler after server initialization");
        } else if !path.starts_with('/') {
            panic!("Path {} must start with '/'", path);
        } else if path.ends_with('/') {
            panic!("Path {} cannot end with '/'", path);
        } else {
            let path: Vec<&str> = path.split("/").skip(1).collect();
            self.handler_tree.add(method, &path, handler);
        }
    }

    // Handler adders
    pub fn get(&mut self, path: &str, handler: PathHandler) -> () {
        self.add_handler(
            "GET",
            path,
            handler
        )
    }
    pub fn post(&mut self, path: &str, handler: PathHandler) -> () {
        self.add_handler(
            "POST",
            path,
            handler
        )
    }
    pub fn all(&mut self, path: &str, handler: PathHandler) -> () {
        self.add_handler(
            "*",
            path,
            handler
        )
    }
    pub fn other(&mut self, method: &str, path: &str, handler: PathHandler) -> () {
        self.add_handler(
            method,
            path,
            handler
        )
    }
}