use std::io::prelude::*;
use std::net::TcpListener;

use super::{Handler, Request, Response};

pub struct Server {
    handlers: Vec<Handler>
}

impl Server {
    pub fn new(handlers: Vec<Handler>) -> Self {
        Server { handlers }
    }
    pub fn listen(self, port: u16) {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut buffer = [0; 1024];
            let mut stream = stream.unwrap();

            stream.read(&mut buffer).unwrap();

            let req = Request::new(String::from_utf8_lossy(&buffer[..]).as_ref());

            println!("{} {}", req.method, req.path);

            for h in &self.handlers {
                if h.path.eq(req.path.as_str()) | h.path.eq("*") {
                    if h.method.is_empty() | h.method.eq(req.method.as_str()) {
                        let res = (h.handler)(&req);
                        if res.is_some() {
                            let Response { status_code, status_text, http_version, body } = res.unwrap();
                            let mut response_text = format!("{} {} {}\r\n\r\n", http_version, status_code, status_text);
                            response_text.push_str(&body);

                            stream.write(response_text.as_bytes()).unwrap();
                            stream.flush().unwrap();
                            break;
                        }
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
}