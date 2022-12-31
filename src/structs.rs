use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello;

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard (Vec<PublicPlayer>);

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64 
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
}