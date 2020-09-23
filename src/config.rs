
use structopt::StructOpt;
use std::path::PathBuf;
use crate::Address;

#[derive(Debug, StructOpt)]
#[structopt(name = "Rustifarm", about = "Asciifarm server in Rust")]
pub struct Config {
	
	#[structopt(short, long, help="A server type and address. Allowed server types: 'inet', 'unix', 'abstract'. Example: \"inet:127.0.0.1:1234\" or \"abstract:rustifarm\"")]
	pub address: Option<Vec<Address>>,
	
	#[structopt(short, long, env="ASCIIFARM_CONTENT_DIR", help="The directory in which the content specifying the world is (maps/encyclopaedia)")]
	pub content_dir: Option<PathBuf>,
	
	#[structopt(short, long, env="ASCIIFARM_SAVE_DIR", help="The directory in which the savegames are")]
	pub save_dir: Option<PathBuf>,
	
	#[structopt(short, long, env="ASCIIFARM_USER_DIR", help="The directory in which the user sign-in data lives")]
	pub user_dir: Option<PathBuf>,
	
	
	
	#[structopt(long, env="USER", help="The name(s) of the server admin(s)")]
	pub admins: String,
	
	#[structopt(long, default_value="100", help="The time (in milliseconds) between two steps")]
	pub step_duration: u64,
	
	#[structopt(long, default_value="300", help="The time (in steps) between two saves")]
	pub save_interval: i64,
	
	#[structopt(long, default_value="300", help="The time (in steps) between the last player leaving a room and the room getting unloaded. Unloading is only done when the rooms are saved, so it could actually take up to save_interval more steps")]
	pub unload_age: i64,
	
}
