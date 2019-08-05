use std::io;
use std::net::{IpAddr};

use grin_core::genesis::{genesis_main};
use grin_p2p::types::{Capabilities, P2PConfig, PeerAddr};

use std::net::{SocketAddr,TcpStream};
use std::{time};
use std::sync::Arc;

use grin_core::pow::Difficulty;
use grin_p2p::Peer;
use grin_p2p::DummyAdapter;

use grin_p2p::handshake::Handshake;
use grin_core::core::hash::Hashed;

fn main(){
	println!("Enter an grin node ip to connenct:");
	let mut ip = String::new();
	io::stdin().read_line(&mut ip).unwrap();
	let ip:IpAddr = ip.trim().parse().unwrap();

    let mut cfg = P2PConfig::default();
    cfg.host = "0.0.0.0".parse().unwrap();
    cfg.port = 12345;

    let net_adapter = Arc::new(DummyAdapter {});

    let addr = SocketAddr::new(ip, 3414);
	let socket = TcpStream::connect_timeout(&addr, time::Duration::from_secs(10)).unwrap();

    let local_addr = PeerAddr(SocketAddr::new(cfg.host, cfg.port));
	let peer = Peer::connect(
		socket,
		Capabilities::UNKNOWN,
		Difficulty::min(),
		local_addr,
	    &Handshake::new(genesis_main().hash(), cfg.clone()),
		net_adapter,
	)
	.unwrap();
    println!("{}: User_agent: {}; Version: {}", addr, &peer.info.user_agent, &peer.info.version);
}