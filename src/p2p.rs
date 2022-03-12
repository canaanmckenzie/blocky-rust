//peer to peer logic

use super::{App, Block};
use log::{error,info};
use serde::{Serialize,Deserialize};
use libp2p::{
	floodsub::{Floodsub,FloodsubEvent,Topic}, //libp2p's publish subscribe protocol
	identity, //identify client in network
	mdns::{Mdns,MdnsEvent},
	swarm::{NetworkBehaviorEventProcess,Swarm},
	NetworkBehavior,
	PeerId,
};


pub static KEYS: Lazy = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy =Lazy::new( || PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy = Lazy::new( || Topic::new("chains")); //simple - broadcasts to all nodes on network client request and our response, this is ipfs but needs security layer
pub static BLOCK_TOPIC: Lazy = Lazy::new( || Topic::new("blocks"));

#[derive(Serialize,Deserialize,Debug)]
pub struct ChainResponse {
	pub blocks: Vec<Block>,
	pub receiver: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LocalChainRequest {
	pub from_peer_id: String,
}

pub enum EventType{
	LocalChainRequest(ChainResponse),
	Input(String),
	Init,
}

#[derive(NetworkBehavior)]
pub struct AppBehavior {
	pub floodsub: Floodsub,
	pub mdns: Mdns,
	#[behavior(ignore)]
	pub response_sender: mspc::UnboundSender,
	#[behavior(ignore)]
	pub init_sender: mpsc::UnboundSender,
	#[behavior(ignore)]
	pub app: App,
}

