use serde::{Serialize,Deserialize};
use chrono::prelude::*;


pub struct App {

    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

//simple consensus criteria, initialize chain empty, ask next block on chain size, if bigger, use theirs
impl App {
    fn new() -> Self{
        Self {blocks: vec![]}
    }

    fn genesis(&mut self){
        //hardcode the first block in chain, change later bootstrapping the chain
        let genesis_block =  Block{
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
        };
        self.blocks.push(genesis_block);
    }

    //add func for adding new blocks
    fn try_add_block(&mut self, block: Block){
        let latest_block = self.blocks.last().expect("There is at least one block");
        if self.is_block_valid(&block, latest_block){
            self.blocks.push(block);
        } else {
            error!("could not add blocks - invalid"); //add error handling other than message, race-conditions invalid state breaks node
        }
    }
}

fn main() {

    println!("Hello, world!");
}


