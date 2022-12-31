mod structs;
mod request;
use std::net::TcpStream;

fn main() {
    test_server();
}

fn test_server() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    
    //send
    let send = structs::Message::Hello;
    request::send(&mut stream, send);
    
    //receive
    let receive = request::receive(&mut stream);
    print!("{}", receive);
}