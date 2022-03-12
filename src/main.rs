use serde::{Serialize,Deserialize};

pub struct App {

    pub blocks: Vec<Block>,
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

impl App {
    fn new() -> Self{
        Self {blocks: vec![]}
    }
}

fn main() {

    println!("Hello, world!");
}


