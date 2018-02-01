
use std::net::{ToSocketAddrs,UdpSocket};

struct Channel {
	max_capacity: usize,
	last_yielded: usize,
	waiting: HashMap<usize, M>
}

pub struct EndPoint<C,S> {
	socket: UdpSocket,
	next_outbound_id: u64,
	channels: HashMap<u8, Channel<M>>,
}

impl ClientEnd<S,C> {

	pub fn new<A: ToSocketAddrs>(a: A, max_buffered: usize) -> Result<ClientEnd, ()> {

	}

	pub fn send(s: &S, dont_lose: bool, delivery: bool, channel: Option<u8>) -> Result<(), SendError> {

	}

	pub fn recv_anything(&mut self, blocking: bool) -> Result<(M, Option<u8>), RecvError> {

	}

	pub fn recv_from_channel(&mut self, blocking: bool) -> Result<M, RecvError> {

	}
}

pub enum RecvError {
	Timeout,
	PeerDisconnect,
	HardDisconnect,

}




///////

enum Sward {
	Ello, Lates, YourMum
}

enum Cward {
	Welcome, Cheerio, GoodnessMe,
}

fn user_end() {
	let addr = "127.0.0.1:8008";
	let client_end = ClientEnd<Cward, Sward>::connect(&addr).expect("Oh noes!");
	client_end.send(&Sward::Ello, false, None).expect("shyet");
}