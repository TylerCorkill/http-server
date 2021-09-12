use tust::{Handler, Response, Server};

mod tust;

fn main() {
    Server::new(vec![
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