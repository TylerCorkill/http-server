use std::collections::HashMap;
use crate::tust::Handler;

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    header_map: HashMap<String, String>,
    pub body: String,
}

#[allow(dead_code)]
impl Request {
    pub fn new(request: &str) -> Self {
        // let (status, rest) = request.split_at(request.find("\r\n").unwrap());
        let (status, rest) = request.split_once("\r\n").unwrap();
        let (_headers, body) = rest.split_once("\r\n\r\n").unwrap();

        let mut status_iter = status.splitn(3, " ");
        let method = status_iter.next().unwrap().to_owned();
        let path = status_iter.next().unwrap().to_owned();
        let http_version = status_iter.next().unwrap().to_owned();

        let mut header_map = HashMap::with_capacity(_headers.lines().count());

        for line in _headers.lines().skip(1) {
            let (header, value) = line.split_at(line.find(':').unwrap());
            header_map.insert(header.to_owned(), value[2..].to_owned());
        }

        Request {
            method,
            path,
            http_version,
            header_map,
            body: body.to_owned(),
        }
    }
    pub fn header(&self, header: &str) -> Option<String> {
        if self.header_map.contains_key(header) {
            Some(self.header_map[header].to_owned())
        } else {
            None
        }
    }
    pub fn matches(&self, handler: &Handler) -> bool {
        (handler.path.eq(self.path.as_str()) | handler.path.eq("*")) &
        (handler.method.is_empty() | handler.method.eq(self.method.as_str()))
    }
}