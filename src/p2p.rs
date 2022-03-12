//peer to peer logic

use super::{App, Block};
use log::{error,info};
use serde::{Serialize,Deserialize};


pub static KEYS: Lazy = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy =Lazy::new( || PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy = Lazy::new( || Topic::new("chains"));
pub static BLOCK_TOPIC: Lazy = Lazy::new( || Topic::new("blocks"));

