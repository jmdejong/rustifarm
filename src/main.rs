

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;
use std::io;

use json;
use json::JsonValue;

pub mod server;
use self::server::Server;
// use self::server::tcpserver::TcpServer;
use self::server::unixserver::UnixServer;


enum Message {
	Name(String),
	Chat(String),
	Input,
	Invalid(String)
}

struct GameServer<T: Server> {
	players: HashMap<usize, String>,
	server: T
}

impl<T: Server> GameServer<T> {
	pub fn new(server: T) -> GameServer<T> {
		GameServer {
			players: HashMap::new(),
			server
		}
	}
	
	fn send_error(&mut self, id: usize, err_text: &str) -> Result<(), io::Error>{
		self.server.send(id, err_text)
	}
	
	pub fn broadcast_message(&mut self, text: &str){
		for (id, _name) in &self.players {
			let _ = self.server.send(*id, text);
		}
	}
	
	pub fn handle_message(&mut self, id: usize, msg: Message) {
		match msg {
			Message::Name(name) => {
				let (firstchar, username) = name.split_at(1);
				if firstchar == "~"{
					if Some(username.to_string()) != self.server.get_name(id) {
						let _ = self.send_error(id, &format!("A tilde name must match your username"));
					}
				} else {
					if let Some(oldname) = self.players.get(&id) {
						self.broadcast_message(&format!("{} is now known as {}", oldname, name));
					} else {
						self.broadcast_message(&format!("{} connected", name));
					}
					self.players.insert(id, name);
				}
			}
			Message::Chat(text) => {
				if let Some(name) = self.players.get(&id) {
					self.broadcast_message(&format!("{}: {}", name, text));
				} else {
					let _ = self.send_error(id, &format!("Set a name before you send other messages"));
				}
			}
			Message::Input => { () }
			Message::Invalid(text) => {
				let _ = self.send_error(id, &format!("Invalid: {}", text));
			}
		}
	}
	
	pub fn remove_connection(&mut self, id: usize) {
		if let Some(name) = self.players.remove(&id){
			self.broadcast_message(&format!("{} disconnected", name));
		}
	}
}

fn main() {
	
// 	let addr = "127.0.0.1:1234".parse().unwrap();

	let addr = Path::new("\0rustifarm");
	
	let mut socketserver = UnixServer::new(&addr).expect("binding server failed");
	
	let mut gameserver = GameServer::new(socketserver);
	
	println!("listening on {:?}", addr);
	
// 	let mut players: HashMap<usize, String> = HashMap::new();
	
	loop {
		let _joined = server.accept_pending_connections();
// 		for id in joined {
// // 			let name = server.get_name(id).expect("Unable to get name");
// 			gameserver.broadcast_message(&format!("{} connected", id));
// 		}
		let (messages, left) = server.recv_pending_messages();
		for (id, message) in messages {
			
// 			let name = server.get_name(id).expect("Unable to get name");
			gameserver.handle_message(parse_message(&message[..]));
// 			match parse_message(&message[..]) {
// 				Message::Name(name) => {
// 					let (firstchar, username) = name.split_at(1);
// 					if firstchar == "~"{
// 						if Some(username.to_string()) != server.get_name(id) {
// 							let _ = server.send(id, &format!("A tilde name must match your username"));
// 						}
// 					} else {
// 						if let Some(oldname) = players.get(&id) {
// 							server.broadcast(&format!("{} is now known as {}", oldname, name));
// 						} else {
// 							server.broadcast(&format!("{} connected", name));
// 						}
// 						players.insert(id, name);
// 					}
// 				}
// 				Message::Chat(text) => {
// 					if let Some(name) = players.get(&id) {
// 						server.broadcast(&format!("{}: {}", name, text));
// 					} else {
// 						let _ = server.send(id, &format!("Set a name before you send other messages"));
// 					}
// 				}
// 				Message::Input => { () }
// 				Message::Invalid(text) => {
// 					let _ = server.send(id, &format!("Invalid: {}", text));
// 				}
// 			};
			println!("{}: {}", id, message);
		}
		for id in left {
			gameserver.remove_connection(id);
// 			if let Some(name) = players.remove(&id){
// 				server.broadcast(&format!("{} disconnected", name));
// 			}
		}
		sleep(Duration::from_millis(100));
	}
}


fn parse_message(msg: &str) -> Message {
	if let Ok(data) = json::parse(msg) {
		if let JsonValue::Array(arr) = data {
			if arr.len() < 2 {
				return Message::Invalid("array not long enough".to_string());
			}
			if let Some(msgtype) = arr[0].as_str() {
				match msgtype {
					"name" => {
						if let Some(name) = arr[1].as_str(){
							Message::Name(name.to_string())
						} else {
							Message::Invalid("name is not a string".to_string())
						}
					}
					"chat" => {
						if let Some(text) = arr[1].as_str(){
							Message::Chat(text.to_string())
						} else {
							Message::Invalid("chat text is not a string".to_string())
						}
						
					}
					"input" => {
						Message::Input
					}
					_ => {
						Message::Invalid(format!("unknown messsage type {:?}", msgtype).to_string())
					}
				}
			} else { Message::Invalid(format!("first array value not string: {:?}", arr[0].dump()).to_string()) }
		} else { Message::Invalid("not json array".to_string()) }
	} else { Message::Invalid("not json message".to_string()) }
}


