
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use structopt::StructOpt;

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
mod parameterexpression;
mod encyclopedia;
mod template;
mod roomtemplate;
mod savestate;
mod playerid;
mod playerstate;
mod roomid;
mod persistence;
mod worldloader;
mod world;
mod sprite;
mod timestamp;
mod purgatory;
mod config;
mod item;
mod exchange;
mod errors;
mod auth;
mod fromtoparameter;

use self::{
	pos::Pos,
	playerid::PlayerId,
	roomid::RoomId,
	item::ItemId,
	errors::{Result, PResult},
	sprite::Sprite,
	template::Template,
	encyclopedia::Encyclopedia,
	timestamp::Timestamp,
	componentwrapper::ComponentWrapper,
	
	gameserver::GameServer,
	server::Server,
	server::address::Address,
	persistence::FileStorage,
	controls::Action,
	worldloader::{WorldLoader, WorldMeta},
	world::World,
	worldmessages::MessageCache
};



fn main(){
	
	let config = config::Config::from_args();
	
	println!("Server admin(s): {}", config.admins);
	
	let adresses = config.address
		.unwrap_or(
			(if cfg!(target_os = "linux") {
				vec!["abstract:rustifarm", "inet:127.0.0.1:9021"]
			} else {
				vec!["inet:127.0.0.1:9021"]
			})
			.iter()
			.map(|a| a.parse().unwrap())
			.collect()
		);
	println!("adresses: {:?}", adresses);
	let servers: Vec<Box<dyn Server>> = 
		adresses
		.iter()
		.map(|a| a.to_server().unwrap())
		.collect();
	
	let user_dir = config.user_dir.unwrap_or(
		auth::FileRegister::default_register_dir().expect("couldn't find any save directory")
	);
	println!("user auth directory: {:?}", user_dir);
	let users = auth::FileRegister::new(user_dir);
	
	let mut gameserver = GameServer::new(servers, Box::new(users), config.admins);
	
	let content_dir = config.content_dir.unwrap_or(
		PathBuf::new()
			.join(std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string()))
			.join("content/")
	);
	println!("content directory: {:?}", content_dir);
	let loader = WorldLoader::new(content_dir);
	let WorldMeta{encyclopediae, default_room} = loader.load_world_meta().expect("Failed to load world meta information");
	
	let mut encyclopedia = Encyclopedia::default();
	for enc in encyclopediae {
		encyclopedia = encyclopedia.merge(
			loader.load_encyclopedia(&enc).expect(&format!("Failed to load encyclopedia {}", enc))
		)
	}
	
	let save_dir = config.save_dir.unwrap_or(
		FileStorage::default_save_dir().expect("couldn't find any save directory")
	);
	println!("save directory: {:?}", save_dir);
	let storage = FileStorage::new(save_dir);

	let mut world = World::new(encyclopedia, loader, Box::new(storage), default_room);
	
	let mut message_cache = MessageCache::default();
	
	// close handler
	// todo: don't let the closing wait on sleep (using a timer thread or recv_timeout)
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	ctrlc::set_handler(move || {
		println!("shutting down");
		r.store(false, Ordering::SeqCst);
	}).expect("can't set close handler");
	
	
	println!("asciifarm started");
	
	
	while running.load(Ordering::SeqCst) {
		let actions = gameserver.update();
		for action in actions {
			match action {
				Action::Input(player, control) => {
					if let Err(err) = world.control_player(player.clone(), control){
						println!("error controlling player {:?}: {:?}", player, err);
					}
				}
				Action::Join(player) => {
					if let Err(err) = world.add_player(&player) {
						println!("Error: can not add player {:?}: {:?}", player, err);
						if let Err(senderr) = gameserver.send_player_error(&player, "worlderror", "invalid room or savefile") {
							println!("Error: can not send error message to {:?}: {:?}", player, senderr);
						}
					}
				}
				Action::Leave(player) => {
					if let Err(err) = world.remove_player(&player) {
						println!("Error: can not remove player {:?}: {:?}", player, err);
					}
					message_cache.remove(&player);
				}
			}
		}
		world.update();
		if world.time.0 % config.save_interval == 0 {
			world.save();
			world.unload_rooms(config.unload_age);
		}
		let messages = world.view();
		for (player, mut message) in messages {
			message_cache.trim(&player, &mut message);
			if message.is_empty(){
				continue;
			}
// 			println!("m {}", message.to_json());
			if let Err(err) = gameserver.send(&player, message.to_json()) {
				println!("Error: failed to send to {:?}: {:?}", player, err);
			}
		}
		
		sleep(Duration::from_millis(config.step_duration));
	}
	println!("saving world");
	world.save();
	println!("world saved");
}




