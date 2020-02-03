

use std::io;
use std::net::SocketAddr;
use mio::net::{TcpListener, TcpStream};
use slab::Slab;

use super::streamconnection::StreamConnection;
use super::Server;


pub struct TcpServer {
	listener: TcpListener,
	connections: Slab<StreamConnection<TcpStream>>
}

impl TcpServer {

	pub fn new(addr: &SocketAddr) -> Result<TcpServer, io::Error> {
		let listener = TcpListener::bind(addr)?;
		Ok( TcpServer {
			listener,
			connections: Slab::new()
		})
	}
}

impl Server for TcpServer {

	fn accept_pending_connections(&mut self) -> Vec<usize> {
		let mut new_connections = Vec::new();
		loop {
			match self.listener.accept() {
				Err(_e) => {
					break;
				}
				Ok((stream, _address)) => {
					let con = StreamConnection::new(stream);
					let id = self.connections.insert(con);
					new_connections.push(id);
				}
			}
		}
		new_connections
	}


	fn recv_pending_messages(&mut self) -> (Vec<(usize, String)>, Vec<usize>){
	// 	let mut buf = [0; 2048];
		let mut messages: Vec<(usize, String)> = Vec::new();
		let mut to_remove = Vec::new();
		for (key, connection) in self.connections.iter_mut(){
			match connection.read() {
				Err(_e) => {
					to_remove.push(key);
				}
				Ok((con_messages, closed)) => {
					for message in con_messages {
						messages.push((key, message));
					}
					if closed {
						to_remove.push(key);
					}
				}
			}
		}
		for key in to_remove.iter() {
			self.connections.remove(*key);
		}
		(messages, to_remove)
	}

	fn broadcast(&mut self, text: &str) {
		for (_id, conn) in self.connections.iter_mut() {
			let _ = conn.send(text);
		}
	}
	
	fn send(&mut self, id: usize, text: &str) -> Result<(), io::Error> {
		match self.connections.get_mut(id){
			Some(conn) => {
				conn.send(text)
			}
			None => Err(io::Error::new(io::ErrorKind::Other, "index is empty"))
		}
	}
	

}

