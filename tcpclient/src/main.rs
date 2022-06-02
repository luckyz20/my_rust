use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write("hello,lisi".as_bytes()).unwrap();
    let mut buf = [0; 10];
    stream.read(&mut buf).unwrap();
    println!("from server: {:?}", str::from_utf8(&buf).unwrap());
}
