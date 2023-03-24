use crate::http::{Request, Response, StatusCode};
use crate::server::Handler;
pub struct WebHandler;

impl Handler for WebHandler {
    fn handle_request(&mut self, _request: &Request) -> Response {
        Response::new(StatusCode::OK, Some("<h1>It works!</h1>".to_string()))
    }
}
