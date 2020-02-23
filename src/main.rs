
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
mod sprite;
mod attack;

pub use self::{
	pos::Pos,
	playerid::PlayerId,
	roomid::RoomId,
	util::Result,
	sprite::Sprite,
	template::Template,
	encyclopedia::Encyclopedia
};

use self::{
	gameserver::GameServer,
	server::unixserver::UnixServer,
	server::tcpserver::TcpServer,
	server::Server,
	defaultencyclopedia::default_encyclopedia,
	persistence::FileStorage,
	controls::Action,
	worldloader::WorldLoader,
	world::World,
	worldmessages::MessageCache
};



fn main() -> Result<()>{
	
	let mut servers: Vec<Box<dyn Server>> = Vec::new();

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr)?;
	servers.push(Box::new(unixserver));
	
	let addr = "127.0.0.1:1234".parse()?;
	let inetserver = TcpServer::new(&addr)?;
	servers.push(Box::new(inetserver));
	
	let mut gameserver = GameServer::new(servers);
	
	
	let loader = WorldLoader::new(PathBuf::from_str(&(std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string()).to_owned() + "/content/maps/"))?);
	
	let storage = FileStorage::new(FileStorage::savedir().expect("couldn't find any save directory"));

	let mut world = World::new(default_encyclopedia(), loader, Box::new(storage), RoomId::from_str("room"));
	
	println!("asciifarm started");
	
	let mut message_cache = MessageCache::default();
	
	let mut count = 0;
	loop {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					let _ = world.control_player(player, control);
				}
				Action::Join(player) => {
					world.add_player(&player)?;
				}
				Action::Leave(player) => {
					world.remove_player(&player)?;
				}
			}
		}
		world.update();
		if count % 50 == 0 {
			world.save();
		}
		let messages = world.view();
		for (player, mut message) in messages {
			message_cache.trim(&player, &mut message);
			if message.is_empty(){
				continue;
			}
			let _ = gameserver.send(&player, message.to_json());
		}
		
		count += 1;
		sleep(Duration::from_millis(100));
	}
}


