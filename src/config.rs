
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
	
}
