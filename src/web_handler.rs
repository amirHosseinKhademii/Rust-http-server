use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;
use std::fs::read_to_string;
pub struct WebHandler {
    public_path: String,
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        read_to_string(path).ok()
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, Some(self.read_file("index.html").unwrap())),
                "/hello" => {
                    Response::new(StatusCode::OK, Some(self.read_file("hello.html").unwrap()))
                }
                _ => Response::new(StatusCode::NotFound, None),
            },

            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
