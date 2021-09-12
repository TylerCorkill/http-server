use crate::server::{Handler, Response};

mod server;

fn main() {
    server::new(vec![
        Handler::get("/test", |req| {
            None
        }),
        Handler::get("/bingo", |req| {
            Some(Response::success(""))
        }),
        Handler::all("*", |req| {
            Some(Response::error("No Handler Found!"))
        }),
    ]).listen(8080);
}