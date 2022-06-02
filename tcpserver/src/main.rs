use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Running on port 8080......");
    for item in listener.incoming() {
        let stream = item.unwrap();
        println!("Connection established!");
        //handle connection
        handle_connection(stream);
    }
}
/*
    处理
*/
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );

    stream.write(&buffer).unwrap();
}
