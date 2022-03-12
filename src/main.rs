use serde::{Serialize,Deserialize};
use chrono::prelude::*;
use log::{error, info, warn};
use sha2::{Digest, Sha256};

//initialize struct to hold chain of blocks
pub struct App {
    pub blocks: Vec<Block>,
}

//initate a block
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

const DIFFICULTY_PREFIX: &str = "00";


//use Sha256 and serde to calculate hash from json
fn calculate_hash(id: u64,  timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id":id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for i in hash {
        res.push_str(&format!("{:b}",i));
    }
    res 
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

    //check if block is valid
    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous_hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
            )
            .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} is not the next block after the latest: {}",block.id,previous_block.id);
            return false;

        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
            )) !=block.hash
        {
            warn!("block with id: {} has invalid hash",block.id);
            return false;
        } 
        true
    }
}


fn main() {

    println!("Hello, world!");
}


