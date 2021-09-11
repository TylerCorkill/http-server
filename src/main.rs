// use std::io
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;






struct Request {
    method: String,
    path: String,
    http_version: String,
    status: String,
    _headers: String,
    _header_map: HashMap<String, String>,
    data: String,
}

impl Request {
    fn new(request: &str) -> Self {
        let (status, rest) = request.split_at(request.find("\r\n").unwrap());
        let (_headers, data) = rest.split_at(rest.find("\r\n\r\n").unwrap());
        let mut status_iter = status.splitn(3, " ");

        let method = status_iter.next().unwrap().to_owned();
        let path = status_iter.next().unwrap().to_owned();
        let http_version = status_iter.next().unwrap().to_owned();


        let mut _header_map = HashMap::with_capacity(_headers.lines().count());

        let mut lines = _headers.lines();
        lines.next();

        for line in lines {
            let (header, value) = line.split_at(line.find(':').unwrap());
            _header_map.insert(header.to_owned(), value[2..].to_owned());
        }

        Request {
            method,
            path,
            http_version,
            status: status.to_owned(),
            _headers: _headers.to_owned(),
            _header_map,
            data: data.to_owned(),
        }
    }
    fn header(self, header: &str) -> Option<String> {
        if self._header_map.contains_key(header) {
            Some(self._header_map[header].to_owned())
        } else {
            None
        }
    }
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut buffer = [0; 1024];
        stream.unwrap().read(&mut buffer).unwrap();

        let req = Request::new(String::from_utf8_lossy(&buffer[..]).as_ref());

        println!("{} {}", req.method, req.path);

        // println!("{}", req.method);
        // println!("{}", req.path);
        // println!("{}", req._headers);
        println!("{}", req.header("Content-Type").unwrap());
        // println!("{}", req.data);
    }
}