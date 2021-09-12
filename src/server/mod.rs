pub use handler::Handler;
pub use request::Request;
pub use response::Response;
use server::Server;

mod handler;
mod request;
mod response;
mod server;

pub fn new(handlers: Vec<Handler>) -> Server { Server::new(handlers) }
