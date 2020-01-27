

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

pub mod server;
pub mod gameserver;
pub mod room;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;

use json;


fn main() {
	

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	
	
	let servers: Vec<Box<dyn Server>> = vec![Box::new(unixserver), Box::new(inetserver)];
	let mut gameserver = GameServer::new(servers);
	
	
	let mut room = room::Room::new((32, 32));
	
	loop {
		let _actions = gameserver.update();
		room.update();
		let (field, mapping) = room.view();
		let updatemsg = create_update_message(room.get_size(), field, mapping);
		let _ = gameserver.broadcast_json(updatemsg);
		sleep(Duration::from_millis(100));
	}
}


fn create_update_message((width, height): (i32, i32), field: Vec<usize>, mapping: Vec<Vec<String>>) -> json::JsonValue {
	let mut updatemsg: json::JsonValue = json::array![
		"world",
		json::array![
			json::array![
				"field",
				json::object!{
					"width" => width,
					"height" => height,
				}
			]
		]
	];
	updatemsg[1][0][1]["field"] = json::from(field);
	updatemsg[1][0][1]["mapping"] = json::from(mapping);
	updatemsg
}
