

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
mod savestate;

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
	
	let mut servers: Vec<Box<dyn Server>> = Vec::new();

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	servers.push(Box::new(unixserver));
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	servers.push(Box::new(inetserver));
	
	let mut gameserver = GameServer::new(servers);
	
	
	let mut room = gen_room();
	
	println!("asciifarm started");
	
	
	let mut count = 0;
	loop {
		let actions = gameserver.update();
		
		room.set_input(actions);
		room.update();
		if count % 20 == 0 {
			println!("{}", room.save().to_json());
		}
		let messages = room.view();
		for (player, message) in messages {
			let _ = gameserver.send(&player, message.to_json());
		}
		
		count += 1;
		sleep(Duration::from_millis(100));
	}
}

fn gen_room<'a, 'b>() -> Room<'a, 'b> {
	let assemblages = default_assemblages();
	let mut room = Room::new(assemblages);

	let roomtemplate = RoomTemplate::from_json(&json!({
		"width": 42,
		"height": 22,
		"spawn": [5, 15],
		"field": [
			"     XXXXXXXXXXXX~~~XXXXXXXXXXXXXXXXXXXXXX",
			"     ,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,,X",
			"    ,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,,X",
			"    ,,,,,,,,,,,,,~~~~,,,,,,,,,,,,,,,,,,,,X",
			" bbbb,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"    ,,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"   ,,,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"  ,,,,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			"X,,,,,,,,,,,,,,,,,~~~~,,,,,,T,,,,,,,,,,,,X",
			"X,,,,,,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,X",
			"X,,,,,,,,,,,,,,,,,,~~~,,,,,T,,,,######,,,X",
			"X,,,,,,,,,,,,,,,,,,bbb,,,,,,,,,,#++++#,,,X",
			"X,,,,,.............bbb...........++++#,,,X",
			"X,**,,.,,,,,,,,,,,,bbb,,,,,,,,,,#++++#,,,X",
			"X,*,*,.,,,,,,,,,,,,~~~,,,T,,,T,,#++++#,,,X",
			"X,,*,,.,,,,,,,,,,,,~~~,,,,,,,,,,######,,,X",
			"X,**,,.,,,,,,,,,,,~~~~,,,,,,,,,,f,,,,f,,,X",
			"X,,*,,.,,,,,,,,,,,~~~''''''''''''''''f'''X",
			"X*,,,,.,,,,,,,,,,,~~~'''''''''''f''''f'''X",
			"X,,,,,.,,,,,,,,,,,~~~'''''''''''ffffff'''X",
			"X,,,,,.,,,,,,,,,,,~~~''''''''''''''''''''X",
			"XXXXX,.,XXXXXXXXXX~~~XXXXXXXXXXXXXXXXXXXXX"
		],
		"mapping": {
			"#": "wall",
			",": "grass",
			".": "ground",
			"~": "water",
			"b": "bridge",
			"+": "floor",
			"'": "greengrass",
			"T": ["grass", "tree"],
			"f": ["grass", "fence"],
			"X": "rock",
			"*": ["grass", "pebble"],
			" ": []
		}
	})).unwrap();
	room.load_from_template(&roomtemplate);
	room
}

fn default_assemblages() -> Encyclopedia {
	Encyclopedia::from_json(json!({
		"wall": {
			"components": ["Blocking"],
			"sprite": "wall",
			"height": 2
		},
		"rock": {
			"components": ["Blocking"],
			"sprite": "rock",
			"height": 10
		},
		"tree": {
			"components": ["Blocking"],
			"sprite": "tree",
			"height": 3
		},
		"fence": {
			"components": ["Blocking"],
			"sprite": "fence",
			"height": 1
		},
		"grass": {
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
				}],
				"Floor"
			]
		},
		"greengrass": {
			"components": [
				["Visible", {
					"sprite": ["random", [
						["string", "grass1"],
						["string", "grass2"],
						["string", "grass3"]
					]],
					"height": ["float", 0.1]
				}],
				"Floor"
			]
		},
		"ground": {
			"components": ["Floor"],
			"sprite": "ground",
			"height": 0.1
		},
		"floor": {
			"components": ["Floor"],
			"sprite": "floor",
			"height": 0.1
		},
		"bridge": {
			"components": [
				"Floor"
			],
			"sprite": "bridge",
			"height": 0.1
		},
		"water": {
			"components": [],
			"sprite": "water",
			"height": 0.1
		},
		"pebble": {
			"components": [
				["Item", {"ent": ["template", "pebble"], "name": ["string", "pebble"]}]
			],
			"sprite": "pebble",
			"height": 0.4
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
				}],
				["Inventory", {"capacity": ["int", 3]}],
				["Health", {"health": ["int", 9], "maxhealth": ["int", 10]}]
			]
		}
	})).unwrap()
}

