use super::router::Router;
use http::httprequest::HttpRequest;
use std::{io::Read, net::TcpListener};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    //new
    pub fn new(socket_addr: &str) -> Server {
        Server {
            socket_addr: socket_addr,
        }
    }

    //run
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for item in connection_listener.incoming() {
            println!("Connection established");
            let mut stream = item.unwrap();

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer);
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
