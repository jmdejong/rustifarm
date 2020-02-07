

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

use serde_json::json;

mod server;
mod gameserver;
mod room;
mod util;
mod controls;
mod components;
mod resources;
mod systems;
mod worldmessages;
mod pos;
mod componentwrapper;
mod parameter;
mod assemblage;
mod componentparameter;
mod encyclopedia;
mod template;
mod roomtemplate;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
use self::room::Room;
use self::util::ToJson;
use self::encyclopedia::Encyclopedia;
use self::template::Template;
pub use self::pos::Pos;



fn main() {
	

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	
	
	let servers: Vec<Box<dyn Server>> = vec![Box::new(unixserver), Box::new(inetserver)];
	let mut gameserver = GameServer::new(servers);
	
	
	let mut room = gen_room(50, 40);
	
	loop {
		let actions = gameserver.update();
		
		room.set_input(actions);
		room.update();
		let messages = room.view();
		for (player, message) in messages {
			let _ = gameserver.send(&player, message.to_json());
		}
		sleep(Duration::from_millis(100));
	}
}

fn gen_room<'a, 'b>(width: i64, height: i64) -> Room<'a, 'b> {
	let assemblages = default_assemblages();
	let mut room = Room::new(assemblages.clone(), (width, height));
	let wall = &Template::empty("wall");
	for x in 0..width {
		room.add_entity(&wall, Pos::new(x, 0)).unwrap();
		room.add_entity(&wall, Pos::new(x, height - 1)).unwrap();
	}
	for y in 1..height-1 {
		room.add_entity(&wall, Pos::new(0, y)).unwrap();
		room.add_entity(&wall, Pos::new(width - 1, y)).unwrap();
	}
	for x in 1..width-1 {
		for y in 1..height-1 {
			let grass = &Template::empty("grass");
			room.add_entity(&grass, Pos::new(x, y)).unwrap();
		}
	}
	room
}

fn default_assemblages() -> Encyclopedia {
	Encyclopedia::from_json(json!({
		"wall": {
			"arguments": [],
			"components": [
				["Blocking", {}],
				["Visible", {
					"sprite": ["string", "wall"],
					"height": ["float", 2.0]
				}]
			]
		},
		"grass": {
			"arguments": [],
			"components": [
				["Visible", {
					"sprite": ["random", [
						["string", "grass1"],
						["string", "grass2"],
						["string", "grass3"],
						["string", "grass1"],
						["string", "grass2"],
						["string", "grass3"],
						["string", "ground"]
					]],
					"height": ["float", 0.1]
				}]
			]
		},
		"player": {
			"arguments": [["name", "string", null]],
			"components": [
				["Visible", {
					"sprite": ["string", "player"],
					"height": ["float", 1.0]
				}],
				["Player", {
					"name": ["arg", "name"]
				}]
			]
		}
	})).unwrap()
}

