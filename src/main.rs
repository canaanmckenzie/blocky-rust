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

//simple mining scheme basis - starts with 00 denotes "difficulty" network for mining a block
//in more complicated network this is a defined network attribute, agreed on between nodes by consensus alg + net hash pwr
//consensus guarantees a new block is prepared in a set amount of time
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

//check whether a hash fits into difficulty prefix condition
fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for i in hash {
        res.push_str(&format!("{:b}",i));
    }
    res 
}

//initiate block and implement simple mining scheme
impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);

        Self {
            id,
            hash,
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }
}

//mine logic - return nonce in hash to verify data 
fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("Mining block..."); 
    let mut nonce = 0;

    //infinite mining loop...add timeout?
    loop {
        if nonce % 100000 == 0 { 
            info!("nonce: {}", nonce); //"ticker - log iterations"
        }

        let hash =  calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);
        //check if hash adheres to difficulty criteria
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mined! nonce: {}, hash: {}, binary_hash: {}", nonce, hex::encode(&hash), binary_hash);
            return (nonce, hex::encode(hash)); //if adheres, log block mined otherwise go again 
        }
        nonce += 1;
    }
}

//simple consensus criteria, initialize chain empty, ask next block on chain size, if bigger, use theirs
impl App {
    fn new() -> Self{
        Self {blocks: vec![]}
    }

    //first block hard coded
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

    //check if chain is valid - if one block fails the entire chain fails ignore genesis block
    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len(){
            if i == 0 {
                continue;
            }
            let first = chain.get(i-1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(second, first) {
                false;
            }
        }
        true
    }

    //choose the longest valid chain
    fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote 
            }
        } else if is_remote_valid && !is_local_valid {
            remote 
        } else if !is_remote_valid && is_local_valid {
            local 
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}


fn main() {

    println!("Hello, world!");
}


