

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

pub mod server;
pub mod gameserver;
pub mod room;
pub mod util;
pub mod controls;
pub mod assemblages;
pub mod components;
pub mod resources;
pub mod systems;
pub mod worldmessages;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
use self::assemblages::{Wall, Grass};

use serde_json;


fn main() {
	

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	
	
	let servers: Vec<Box<dyn Server>> = vec![Box::new(unixserver), Box::new(inetserver)];
	let mut gameserver = GameServer::new(servers);
	
	
	let mut room = room::Room::new((50, 50));
	
	gen_room(&mut room);
	
	loop {
		let actions = gameserver.update();
		
		room.set_input(actions);
		room.update();
		let (field, mapping) = room.view();
		let updatemsg = create_update_message(room.get_size(), field, mapping);
		gameserver.broadcast(updatemsg.as_str());
		sleep(Duration::from_millis(100));
	}
}

fn gen_room(room: &mut room::Room){
	
	let (width, height) = room.get_size();
	let wall = Wall{};
	for x in 0..width {
		room.add_obj(&wall, (x, 0));
		room.add_obj(&wall, (x, height - 1));
	}
	for y in 1..height-1 {
		room.add_obj(&wall, (0, y));
		room.add_obj(&wall, (width - 1, y));
	}
	for x in 1..width-1 {
		for y in 1..height-1 {
			room.add_obj(&Grass::new(), (x, y));
		}
	}
}


fn create_update_message((width, height): (i32, i32), field: Vec<usize>, mapping: Vec<Vec<String>>) -> String {
	let updatemsg= serde_json::json!([
		"world",
		[
			[
				"field",
				{
					"width": width,
					"height": height,
					"field": field,
					"mapping": mapping
				}
			]
		]
	]);
	updatemsg.to_string()
}
