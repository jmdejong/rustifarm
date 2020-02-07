

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

pub use self::pos::Pos;
use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
use self::room::Room;
use self::util::ToJson;
use self::encyclopedia::Encyclopedia;
use self::roomtemplate::RoomTemplate;



fn main() {
	

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	
	
	let servers: Vec<Box<dyn Server>> = vec![Box::new(unixserver), Box::new(inetserver)];
	let mut gameserver = GameServer::new(servers);
	
	
	let mut room = gen_room();
	
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

fn gen_room<'a, 'b>() -> Room<'a, 'b> {
	let assemblages = default_assemblages();
	let mut room = Room::new(assemblages.clone());

	let roomtemplate = RoomTemplate::from_json(&json!({
		"width": 42,
		"height": 22,
		"spawn": [35, 5],
		"field": [
			"##########################################",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#   ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,#",
			"#####,,,##################################"
		],
		"mapping": {
			"#": "wall",
			",": "grass",
			" ": []
		}
	})).unwrap();
	room.load_from_template(&roomtemplate);
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

