use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

pub struct HeaderMap {
    header_map: HashMap<String, String>,
}

#[allow(dead_code)]
impl HeaderMap {
    pub fn new() -> Self {
        HeaderMap {
            header_map: HashMap::new()
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        HeaderMap {
            header_map: HashMap::with_capacity(capacity)
        }
    }
    pub fn get(&self, header: &str) -> Option<&str> {
        if self.header_map.contains_key(header) {
            Some(&self.header_map[header])
        } else {
            None
        }
    }
    pub fn set(&mut self, header: &str, value: &str) -> () {
        self.header_map.insert(header.to_owned(), value.to_owned());
    }
}

impl Display for HeaderMap {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO [2] Header Map formatter performance
        let mut headers = String::new();
        for (header, value) in self.header_map.iter() {
            headers.push_str(&format!("{}: {}\r\n", header, value));
        }
        write!(f, "{}", headers)
    }
}