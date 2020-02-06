

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;

use serde_json::json;

mod server;
mod gameserver;
mod room;
mod util;
mod controls;
mod assemblages;
mod components;
mod resources;
mod systems;
mod worldmessages;
mod pos;
mod assemblage;
// mod load;
mod componentwrapper;
mod parameter;
mod template;
mod componentparameter;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
use self::assemblages::{Grass};
use self::room::Room;
use self::util::ToJson;
use self::template::{Template};



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

fn gen_room<'a, 'b>(width: i32, height: i32) -> Room<'a, 'b> {
	let mut room = Room::new((width, height));
	let assemblages = default_assemblages();
	let wall = assemblages["wall"].instantiate(Vec::new(), HashMap::new()).unwrap();
	for x in 0..width {
		room.add_complist(&wall, (x, 0));
		room.add_complist(&wall, (x, height - 1));
	}
	for y in 1..height-1 {
		room.add_complist(&wall, (0, y));
		room.add_complist(&wall, (width - 1, y));
	}
	for x in 1..width-1 {
		for y in 1..height-1 {
			room.add_obj(&Grass::new(), (x, y));
		}
	}
	room
}

fn default_assemblages() -> HashMap<String, Template> {
	json!({
		"wall": {
			"arguments": [],
			"components": [
				["Blocking", {}],
				["Visible", {
					"sprite": ["string", "wall"],
					"height": ["float", 1.0]
				}]
			]
		},
		"grass": {
			"arguments": [
				["sprite", "string", "grass1"]
			],
			"components": [
				["Visible", {
					"sprite": ["arg", "sprite"],
					"height": ["float", 0.1]
				}]
			]
		}
	}).as_object().unwrap().into_iter().map(|(k, v)| (k.clone(), Template::from_json(v).unwrap())).collect()
}

