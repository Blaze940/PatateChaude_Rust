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

    fn new(input: self::Input) -> Self {

        Self.complexity = input.complexity;
        Self.message = input.message;

        let mut hasher = Md5::new();
        hasher.update(format!("{0}", Self.message));

        let hash = hasher.finalize();

        Self

    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
    
}
