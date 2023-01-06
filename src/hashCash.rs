mod hashCashStruct;

use crate::Challenge;
use crate::structs::{MD5HashCashInput, MD5HashCashOutput};
use md5::{Md5, Digest};

struct HashCash{
    complexity: u64,
    message: String,
    seed: u64,
    hashcode: String
}


impl Challenge for HashCash {

    type Input = MD5HashCashInput;
    
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    fn new(input: Input) -> Self {todo!()}

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
    
}
