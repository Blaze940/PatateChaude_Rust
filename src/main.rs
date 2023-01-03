mod structs;
mod request;
use std::net::TcpStream;
use rand::Rng;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    request::send(&mut stream, structs::Message::Hello);

    let subscribe = structs::Subscribe { name: "test".to_string() };
    request::send(&mut stream, structs::Message::Subscribe(subscribe));



    //Test
    let challenge_complexity = 10;

    let sentence = match challenge_complexity{
        0 => "C'est chou" ,
        //TODO random sequences of characters all distincts,
        1..=16 => "C'est chouette" ,
        17 => "Il fait froid",
        //TODO randam sentences with words of dictionnary DATA
        _ => "Il fait froid et il pleut",
    };

    println!("sentence: {}", sentence);
}