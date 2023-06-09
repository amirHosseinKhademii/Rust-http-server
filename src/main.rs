#![allow(dead_code)]
use server::Server;
use std::env;
use web_handler::WebHandler;

mod http;
mod server;
mod web_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // PUBLIC_PATH=$(pwd)/public cargo run
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebHandler::new(public_path));
}
