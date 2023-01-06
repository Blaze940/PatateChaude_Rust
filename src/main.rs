mod structs;
mod request;
mod hashCash;

use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    request::send(&mut stream, structs::Message::Hello);

    let subscribe = structs::Subscribe { name: "test".to_string() };
    request::send(&mut stream, structs::Message::Subscribe(subscribe));
}

trait Challenge {

    type Input;

    type Output;

    fn name() -> String;

    fn new(input: String) ->   Self;

    fn solve(&self) -> Self::Output;

    fn verify(&self, answer: &Self::Output) -> bool;

}