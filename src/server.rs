use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listen = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listen.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request received: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::OK,
                                        Some("<h1>It works!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Error: {}", e);
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
