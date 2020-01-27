

use std::collections::HashMap;
use std::io;

use json;
use json::JsonValue;

use super::server::Server;


#[derive(Debug)]
enum Message {
	Name(String),
	Chat(String),
	Input(JsonValue),
	Invalid(String)
}

#[derive(Debug)]
pub enum Action {
	Join(String),
	Leave(String),
	Input(String, JsonValue)
}

pub struct GameServer {
	players: HashMap<(usize, usize), String>,
	connections: HashMap<String, (usize, usize)>,
	servers: Vec<Box<dyn Server>>
}

impl GameServer {
	pub fn new(servers: Vec<Box<dyn Server>>) -> GameServer {
		GameServer {
			players: HashMap::new(),
			connections: HashMap::new(),
			servers
		}
	}
	
	pub fn update(&mut self) -> Vec<Action>{
		for server in self.servers.iter_mut(){
			let _ = server.accept_pending_connections();
		}
		
		let mut actions: Vec<Action> = Vec::new();
		let mut input = Vec::new();
		for (serverid, server) in self.servers.iter_mut().enumerate(){
			let (messages, left) = server.recv_pending_messages();
			input.push((serverid, messages, left));
		}
		for (serverid, messages, left) in input {
			for (id, message) in messages {
				let r = self.handle_message((serverid, id), parse_message(&message));
				if let Some(action) = r {
					actions.push(action);
				}
			}
			for id in left {
				if let Some(name) = self.players.remove(&(serverid, id)){
					self.connections.remove(&name);
					self.broadcast_message(&format!("{} disconnected", name));
					actions.push(Action::Leave(name.clone()));
				}
			}
		}
		actions
	}
	
	fn send_error(&mut self, (serverid, connectionid): (usize, usize), errname: &str, err_text: &str) -> Result<(), io::Error>{
		self.servers[serverid].send(connectionid, &json::stringify(json::array!["error", errname, err_text]))
	}
	
	pub fn broadcast_message(&mut self, text: &str){
		println!("m {}", text);
		self.broadcast_json(json::array!["message", text, ""]);
	}
	
	pub fn broadcast_json(&mut self, value: JsonValue){
		let jsontext = json::stringify(value);
		for ((serverid, id), _name) in &self.players {
			let _ = self.servers[*serverid].send(*id, &jsontext);
		}
	}
	
	pub fn send(&mut self, playername: &str, value: JsonValue) -> Result<(), io::Error> {
		let jsontext = json::stringify(value);
		match self.connections.get(playername) {
			Some((serverid, id)) => {
				self.servers[*serverid].send(*id, &jsontext)
			}
			None => Err(io::Error::new(io::ErrorKind::Other, "unknown player name"))
		}
	}
	
	fn handle_message(&mut self, (serverid, connectionid): (usize, usize), msg: Message) -> Option<Action> {
		let id = (serverid, connectionid);
		match msg {
			Message::Name(name) => {
				if name.len() > 256 {
					let _ = self.send_error(id, "invalidname", "A name can not be longer than 256 bytes");
					return None
				}
				if name.len() == 0 {
					let _ = self.send_error(id, "invalidname", "A name must have at least one character");
					return None
				}
				let (firstchar, username) = name.split_at(1);
				if firstchar == "~" {
					if Some(username.to_string()) != self.servers[serverid].get_name(connectionid) {
						let _ = self.send_error(id, "invalidname", &format!("A tilde name must match your username"));
						return None;
					}
				}
				if self.players.contains_key(&id) {
					let _ = self.send_error(id, "invalidaction", &format!("You can not change your name"));
					return None;
				}
				if self.connections.contains_key(&name) {
					let _ = self.send_error(id, "nametaken", &format!("Another connections to this player exists already"));
					return None;
				}
				self.broadcast_message(&format!("{} connected", name));
				self.players.insert(id, name.clone());
				self.connections.insert(name.clone(), id);
				Some(Action::Join(name))
			}
			Message::Chat(text) => {
				if let Some(nameref) = self.players.get(&id) {
					let name = nameref.clone();
					self.broadcast_message(&format!("{}: {}", name, text));
				} else {
					let _ = self.send_error(id, "invalidaction", &format!("Set a name before you send other messages"));
				}
				None
			}
			Message::Input(inp) => {
				if let Some(nameref) = self.players.get(&id) {
					let name = nameref.clone();
					Some(Action::Input(name, inp))
				} else {
					let _ = self.send_error(id, "invalidaction", &format!("Set a name before you send other messages"));
					None
				}
			}
			Message::Invalid(text) => {
				let _ = self.send_error(id, "invalidmessage", &format!("Invalid: {}", text));
				None
			}
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
							Message::Chat(text.escape_debug().to_string())
						} else {
							Message::Invalid("chat text is not a string".to_string())
						}
						
					}
					"input" => {
						Message::Input(arr[1].clone())
					}
					_ => {
						Message::Invalid(format!("unknown messsage type {:?}", msgtype).to_string())
					}
				}
			} else { Message::Invalid(format!("first array value not string: {:?}", arr[0].dump()).to_string()) }
		} else { Message::Invalid("not json array".to_string()) }
	} else { Message::Invalid("not json message".to_string()) }
}


