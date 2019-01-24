

use std::collections::HashMap;
use std::io;

use json;
use json::JsonValue;

use super::server::Server;


pub enum Message {
	Name(String),
	Chat(String),
	Input,
	Invalid(String)
}

pub struct GameServer<T: Server> {
	players: HashMap<usize, String>,
	connections: HashMap<String, usize>,
	server: T
}

impl<T: Server> GameServer<T> {
	pub fn new(server: T) -> GameServer<T> {
		GameServer {
			players: HashMap::new(),
			connections: HashMap::new(),
			server
		}
	}
	
	pub fn update(&mut self) {
		self.accept_connections();
		self.receive_messages();
	}
	
	fn accept_connections(&mut self) {
		let _joined = self.server.accept_pending_connections();
	}
	
	fn receive_messages(&mut self) {
		
		let (messages, left) = self.server.recv_pending_messages();
		for (id, message) in messages {
			self.handle_message(id, parse_message(&message));
		}
		for id in left {
			self.remove_connection(id);
		}
	}
	
	fn send_error(&mut self, id: usize, errname: &str, err_text: &str) -> Result<(), io::Error>{
		self.server.send(id, &json::stringify(json::array!["error", errname, err_text]))
	}
	
	pub fn broadcast_message(&mut self, text: &str){
		println!("{}", text);
		let jsontext = json::stringify(json::array!["message", text]);
		for (id, _name) in &self.players {
			let _ = self.server.send(*id, &jsontext);
		}
	}
	
	pub fn handle_message(&mut self, id: usize, msg: Message) {
		match msg {
			Message::Name(name) => {
				let (firstchar, username) = name.split_at(1);
				if firstchar == "~"{
					if Some(username.to_string()) != self.server.get_name(id) {
						let _ = self.send_error(id, "invalidname", &format!("A tilde name must match your username"));
						return;
					}
				}
				if self.players.contains_key(&id) {
					let _ = self.send_error(id, "invalidaction", &format!("You can not change your name"));
					return;
				}
				if self.connections.contains_key(&name) {
					let _ = self.send_error(id, "nametaken", &format!("Another connections to this player exists already"));
					return;
				}
				self.broadcast_message(&format!("{} connected", name));
				self.players.insert(id, name.clone());
				self.connections.insert(name, id);
			}
			Message::Chat(text) => {
				if let Some(name) = self.players.get(&id) {
					self.broadcast_message(&format!("{}: {}", name, text));
				} else {
					let _ = self.send_error(id, "invalidaction", &format!("Set a name before you send other messages"));
				}
			}
			Message::Input => { () }
			Message::Invalid(text) => {
				let _ = self.send_error(id, "invalidmessage", &format!("Invalid: {}", text));
			}
		}
	}
	
	pub fn remove_connection(&mut self, id: usize) {
		if let Some(name) = self.players.remove(&id){
			self.connections.remove(&name);
			self.broadcast_message(&format!("{} disconnected", name));
		}
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


