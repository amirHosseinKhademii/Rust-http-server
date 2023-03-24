use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;
pub struct WebHandler;

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, Some("<h1>Index</h1>".to_string())),
                "/hello" => Response::new(StatusCode::OK, Some("<h1>Hello</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },

            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
