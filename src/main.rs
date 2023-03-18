use http::request::Request;
use server::Server;

mod server;
mod http {
    pub mod method;
    pub mod request;
}

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
