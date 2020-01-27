

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
// use std::net::SocketAddr;

pub mod server;
pub mod gameserver;
// pub mod simpleworld;
pub mod room;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
// use self::simpleworld::{Room, Pos, GameObject};

use json;


fn main() {
	

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	
	
	let servers: Vec<Box<dyn Server>> = vec![Box::new(unixserver), Box::new(inetserver)];
	let mut gameserver = GameServer::new(servers);
	
// 	println!("listening on {:?}", addr);
	
// 	let mut players: HashMap<usize, String> = HashMap::new();
	
// 	let (mut world, dispatcher) = room::make_room((32, 32));
	
	
	let mut room = room::Room::new((32, 32));
// 	dispatcher.dispatch(&mut world);
// 	world.maintain();
	
// 	let mut world = generate_world(32, 32);
	
	loop {
		let _actions = gameserver.update();
		room.update();
		let (field, mapping) = room.view();
		let updatemsg = create_update_message(room.get_size(), field, mapping);
// 		dispatcher.dispatch(&mut world);
// 		world.maintain();
// 		let topview = *world.fetch::<TopView>();
		let _ = gameserver.broadcast_json(updatemsg);
// 		update(&mut gameserver, &mut world);
		sleep(Duration::from_millis(100));
	}
}


// fn generate_world(width: i32, height: i32) -> Room {
// 
// 	let mut world = Room::new((Pos(0,0), Pos(width, height)));
// 	let grass = GameObject::new("grass1");
// 	let wall = GameObject::new("wall");
// 	for x in 0..width {
// 		world.add_obj(Pos(x, 0), wall.clone());
// 		world.add_obj(Pos(x, height -1), wall.clone());
// 	}
// 	for y in 1..height {
// 		world.add_obj(Pos(0, y), wall.clone());
// 		world.add_obj(Pos(width -1, y), wall.clone());
// 	}
// 	for x in 10..20 {
// 		for y in 15 .. 25 {
// 			let pos = Pos(x, y);
// 			world.add_obj(pos, grass.clone());
// 		}
// 	}
// 	world
// }


fn create_update_message((width, height): (i32, i32), field: Vec<usize>, mapping: Vec<Vec<String>>) -> json::JsonValue {
	let mut updatemsg: json::JsonValue = json::array![
		"world",
		json::array![
			json::array![
				"field",
				json::object!{
					"width" => width,
					"height" => height,
// 					"field" => jfield,
// 					"mapping" => json::from(mapping)
				}
			]
		]
	];
	updatemsg[1][0][1]["field"] = json::from(field);
	updatemsg[1][0][1]["mapping"] = json::from(mapping);
	updatemsg
}

// fn update(gameserver: &mut GameServer, world: &mut Room) {
// 	let actions = gameserver.update();
// 	for action in actions {
// 		println!("a {:?}", action);
// 	}
// 	let (_start, Pos(width, height)) = world.area;
// 	let (field, mapping) = world.draw();
// // 	let jfield = json::from(field);
// 	let _ = gameserver.broadcast_json(updatemsg);
// }
