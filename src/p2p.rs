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
	pub response_sender: mspc::UnboundedSender,
	#[behavior(ignore)]
	pub init_sender: mpsc::UnboundedSender,
	#[behavior(ignore)]
	pub app: App,
}

impl AppBehavior {
	pub async fn new(
		app: App,
		response_sender: mpsc::UnboundedSender,
		init_sender: mpsc::UnboundedSender,
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
//if new node is discovered add it to list, if expired delete
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
impl NetworkBehaviorEventProcess for AppBehavior {
	fn inject_event(&mut self, event: FloodsubEvent){
		if let FloodsubEvent::Message(msg) = event {

			if let Ok(resp) = serde_json::from_slice::(&msg.data){
				if resp.receiver == PEER_ID.to_string() {
					info!("Response from {}:",msg.source);
					resp.blocks.iter().for_each(|r| info!(":{?}", r));

					self.app.blocks =  self.apps.choose_chain(self.app.blocks.clone(),resp.blocks); 
				}

			} else if let Ok(resp) = serde_json::from_slice::(&msg.data) {
				info!("sending local chain to {}",msg.source.to_string());
				let peer_id = resp.from_peer_id; 

				if PEER_ID.to_string() == peer_id { 
					if let Err(e) = self.response_sender.send(ChainResponse{
						blocks: self.app.blocks.clone(),
						receiver: msg.source.to_string(),
					}) {
						error!("error sending response via channel {}",e);
					}
				}

			} else if let Ok(block) = serde_json::from_slice::(&msg.data){ //check if new mined block is valid and add it to local chain
				info!("received new block from {}",msg.source.to_string());
				self.app.try_add_block(block);

			}
		}
	}
}

