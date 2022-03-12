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

//keep application in sync with incoming and outgoing network traffic
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

impl AppBehavior {
	pub async fn new(
		app: App,
		response_sender: mpsc::UnboundSender,
		init_sender: mpsc::UnboundSender,
	) -> Self {
		let mut behavior = Self {
			app,
			floodsub: Floodsub::new(*PEER_ID),
			mdns: Mdns::new(Default::default())
			.await
			.expect("can create mdns"),
			response_sender,
			init_sender,
		};
		behavior.floodsub.subscibe(CHAIN_TOPIC.clone());
		behavior.floodsub.subscibe(BLOCK_TOPIC.clone());

		behavior
	}
}

//use multicast dns handler from libp2p
impl NetworkBehaviorEventProcess<MdnsEvent> for AppBehavior {
	fn inject_event(&mut self, event: MdnsEvent){
		match event {
			MdnsEvent::Discovered(discovered_list) => {
				for (peer, _addr) in discovered_list {
					self.floodsub.add_node_to_partial_view(peer);
				}
			}
			MdnsEvent::Expired(expired_list) => {
				for (peer, _addr) in expired_list {
					if !self.mdns.has_node(&peer) {
						self.floodsub.remove_node_from_partial_view(&peer);
					}
				}
			}
		}
	}
}

//incoming event handler
imp NetworkBehaviorEventProcess for AppBehavior {
	
}

