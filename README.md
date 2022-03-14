
An implementation of a blockchain in Rust demonstrating the use of libp2p


TO DO:

+ add persistence when finished

+ add error handling if block fails to add to node

+ consensus between nodes to prevent simultaneous node creation aka no two nodes #4 pointing to #3 for example, create "retry mechanism"
	- rudimentary consensus determines longest "most up to date chain from either local and remote and uses that"

+ security layer between connecting nodes, currently client request and node response are broadcast through the entire network
	- update with libp2p point-to-point request/response model to improve performance...see libp2p docs request_response
	- refactor with gossipsub for more efficiency

Install locally

To run:

install rust and cargo

install nightly cargo for up to date dependencies

cd directory blocky-rust

 ~$:  RUST_LOG=info cargo +nightly run


commands:

+ ls p - list peer address
+ ls c - list local chain
+ create b $data - mines a block with data $data and adds it to chain

- Simple test:

open multiple terminal windows/tabs and switch between them updating and creating blocks and checking the persistence across each 'peer'

