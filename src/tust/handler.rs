use crate::tust::{Request, Response};

pub type PathHandler = fn(&mut Request, &mut Response) -> ();

pub struct Handler {
    pub method: String,
    pub path: String,
    pub handler: PathHandler
}

#[allow(dead_code)]
impl Handler {
    pub fn matches(&self, req: &Request) -> bool {
        (self.path.eq(req.path.as_str()) | self.path.eq("*")) &
        (self.method.is_empty() | self.method.eq(req.method.as_str()))
    }
}