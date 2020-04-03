use std::io;

pub mod tcpserver;
pub mod unixserver;
pub mod address;

mod streamconnection;


pub trait Server {
	
	fn accept_pending_connections(&mut self) -> Vec<usize>;
	
	fn recv_pending_messages(&mut self) -> (Vec<(usize, String)>, Vec<usize>);
	
	fn send(&mut self, id: usize, text: &str) -> Result<(), io::Error>;
	
	fn broadcast(&mut self, text: &str);
	
	fn get_name(&self, _id: usize) -> Option<String> {
		None
	}
}
