use std::collections::HashMap;

use crate::tust::HeaderMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub path_variables: HashMap<String, String>,
    pub http_version: String,
    pub header: HeaderMap,
    pub body: String,
}

#[allow(dead_code)]
impl Request {
    pub fn new(request: &str) -> Self {
        let (status, rest) = request.split_once("\r\n").unwrap();
        let (header_str, body) = rest.split_once("\r\n\r\n").unwrap();

        let mut status_iter = status.splitn(3, " ");
        let method = status_iter.next().unwrap().to_owned();
        let path = status_iter.next().unwrap().to_owned();
        let http_version = status_iter.next().unwrap().to_owned();

        let mut header = HeaderMap::with_capacity(header_str.lines().skip(1).count());

        for line in header_str.lines().skip(1) {
            let (h, value) = line.split_at(line.find(':').unwrap());
            header.set(h, &value[2..]);
        }

        Request {
            method,
            path,
            path_variables: HashMap::new(),
            http_version,
            header,
            body: body.to_owned(),
        }
    }
}