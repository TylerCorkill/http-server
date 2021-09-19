use std::fmt::{self, Display, Formatter};

use crate::tust::HeaderMap;

pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub http_version: String,
    pub header: HeaderMap,
    pub body: String,
    pub complete: bool
}

#[allow(dead_code)]
impl Response {
    pub fn new() -> Self {
        Response {
            status_code: 200,
            status_text: "OK".to_owned(),
            header: HeaderMap::new(),
            body: "".to_owned(),
            http_version: "HTTP/1.1".to_owned(),
            complete: false
        }
    }
    pub fn success(&mut self, body: &str) -> () {
        self.status_code = 200;
        self.status_text = "OK".to_owned();
        self.body = body.to_owned();
        self.http_version = "HTTP/1.1".to_owned();
        self.complete = true;
    }
    pub fn error(&mut self, body: &str) -> () {
        self.status_code = 500;
        self.status_text = "ERROR".to_owned();
        self.body = body.to_owned();
        self.http_version = "HTTP/1.1".to_owned();
        self.complete = true;
    }
}

impl Display for Response {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if self.body.is_empty() {
            write!(
                formatter,
                "{} {} {}\r\n{}\r\n",
                self.http_version,
                self.status_code,
                self.status_text,
                self.header
            )
        } else {
            write!(
                formatter,
                "{} {} {}\r\n{}\r\n{}",
                self.http_version,
                self.status_code,
                self.status_text,
                self.header,
                self.body
            )
        }
    }
}