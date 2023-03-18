use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        let listen = TcpListener::bind(&self.addr).unwrap();
        loop {
            // match listen.accept() {
            //     Ok((mut socket, _)) => {
            //         let mut buffer = [0; 1024];
            //         match socket.read(&mut buffer) {
            //             Ok(_) => {
            //                 println!("Received a request: {}", String::from_utf8_lossy(&buffer));
            //             }
            //             Err(e) => println!("Failed to read from connection: {}", e),
            //         }
            //     }
            //     Err(e) => println!("Failed to establish a connection: {}", e),
            // }
        }
    }
}
