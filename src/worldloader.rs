
use std::path::PathBuf;
use std::fs;
use serde_json;
use serde_json::Value;
use crate::{
	RoomId,
	roomtemplate::RoomTemplate,
	util::Result
};


pub struct WorldLoader {
	pub directory: PathBuf
}

impl WorldLoader {
	pub fn new(path: PathBuf) -> Self {
		Self {
			directory: path
		}
	}
	
	pub fn load_room(&self, id: RoomId) -> Result<RoomTemplate> {
		let mut path = self.directory.clone();
		let fname = id.to_string() + ".json";
		path.push(fname);
		println!("PATH: {:?}", path);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		let template = RoomTemplate::from_json(&json)?;
		Ok(template)
	}
}
