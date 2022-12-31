use crate::structs::Message;
use std::net::TcpStream;
use std::io::prelude::*;

pub fn send (stream: &mut TcpStream, msg: Message) {
    let json = serde_json::to_string(&msg).unwrap();
    let len = json.len() as u32;
    
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(&json.as_bytes()).unwrap();
}

pub fn receive (stream: &mut TcpStream) -> String {
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap();
    let len = u32::from_be_bytes(buf_len);
    
    let mut buf = vec![0u8; len as usize];
    stream.read_exact(buf.as_mut()).unwrap();
    let s = String::from_utf8_lossy(&buf);
    
    return s.to_string();
}