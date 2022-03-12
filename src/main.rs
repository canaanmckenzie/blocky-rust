
fn main() {

use serde::{Serialize,Deserialize};


pub struct App {
    
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}



    println!("Hello, world!");
}
