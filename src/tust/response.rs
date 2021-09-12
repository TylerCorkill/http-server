pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub body: String,
    pub http_version: String
}

impl Response {
    pub fn success(body: &str) -> Self {
        Response {
            status_code: 200,
            status_text: "OK".to_owned(),
            body: body.to_owned(),
            http_version: "HTTP/1.1".to_owned()
        }
    }
    pub fn error(body: &str) -> Self {
        Response {
            status_code: 500,
            status_text: "ERROR".to_owned(),
            body: body.to_owned(),
            http_version: "HTTP/1.1".to_owned()
        }
    }
}