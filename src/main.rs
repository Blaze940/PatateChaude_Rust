use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    
    //send
    let message = "\"Hello\"";
    let len = message.len() as u32;
    
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(message.as_bytes()).unwrap();

    //receive
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap();
    let len = u32::from_be_bytes(buf_len);
    
    let mut buf = vec![0u8; len as usize];
    stream.read_exact(buf.as_mut()).unwrap();
    let s = String::from_utf8_lossy(&buf);
    print!("{}", s);
}
