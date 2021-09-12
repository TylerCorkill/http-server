use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    status: String,
    header_map: HashMap<String, String>,
    pub body: String,
}

#[allow(dead_code)]
impl Request {
    pub fn new(request: &str) -> Self {
        let (status, rest) = request.split_at(request.find("\r\n").unwrap());
        let (_headers, data) = rest.split_at(rest.find("\r\n\r\n").unwrap());

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
            status: status.to_owned(),
            header_map,
            body: data.to_owned(),
        }
    }
    pub fn header(self, header: &str) -> Option<String> {
        if self.header_map.contains_key(header) {
            Some(self.header_map[header].to_owned())
        } else {
            None
        }
    }
}