mod structs;
mod request;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    send_request(&mut stream, structs::Message::Hello);
    send_request(&mut stream, structs::Message::Subscribe(structs::Subscribe { name: "test".to_string() }));
}

fn send_request(stream: &mut TcpStream, msg: structs::Message) {
    //send
    request::send(stream, msg);
    
    //receive
    let receive = request::receive(stream);
    println!("{}", receive);
}