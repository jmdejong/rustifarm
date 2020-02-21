
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

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
mod playerid;
mod defaultencyclopedia;
mod playerstate;
mod roomid;
mod persistence;
mod worldloader;
mod world;

pub use self::pos::Pos;
pub use self::playerid::PlayerId;
pub use self::roomid::RoomId;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;
use self::util::ToJson;
use self::defaultencyclopedia::default_encyclopedia;
use self::persistence::FileStorage;
use crate::controls::Action;
use crate::worldloader::WorldLoader;
use crate::world::World;



fn main() {
	
	let mut servers: Vec<Box<dyn Server>> = Vec::new();

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	servers.push(Box::new(unixserver));
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	servers.push(Box::new(inetserver));
	
	let mut gameserver = GameServer::new(servers);
	
	let loader = WorldLoader::new(PathBuf::from_str(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/content/maps/")).unwrap());
	
	let storage = FileStorage::new(FileStorage::savedir().expect("couldn't find any save directory"));

	let mut world = World::new(default_encyclopedia(), loader, Box::new(storage), RoomId::from_str("room"));
	
	println!("asciifarm started");
	
	
	let mut count = 0;
	loop {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					let _ = world.control_player(player, control);
				}
				Action::Join(player) => {
					world.add_player(&player).unwrap();
				}
				Action::Leave(player) => {
					world.remove_player(&player).unwrap();
				}
			}
		}
		world.update();
		if count % 50 == 0 {
			world.save();
		}
		let messages = world.view();
		for (player, message) in messages {
			let _ = gameserver.send(&player, message.to_json());
		}
		
		count += 1;
		sleep(Duration::from_millis(100));
	}
}


