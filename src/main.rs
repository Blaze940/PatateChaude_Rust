mod structs;
mod request;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    request::send(&mut stream, structs::Message::Hello);

    let subscribe = structs::Subscribe { name: "test".to_string() };
    request::send(&mut stream, structs::Message::Subscribe(subscribe));
}