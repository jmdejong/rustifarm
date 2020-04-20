
use std::path::{PathBuf};
use std::fs;
use serde_json;
use serde_json::Value;
use crate::{
	RoomId,
	roomtemplate::RoomTemplate,
	Result,
	aerr,
	Encyclopedia
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
	
	pub fn load_world_meta(&self) -> Result<WorldMeta> {
		let path = self.directory.join("world.json");
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		let default_room = RoomId::from_str(
			json
				.get("default_room")
				.ok_or(aerr!("world meta does not have default_room"))?
				.as_str()
				.ok_or(aerr!("world meta default_room is not a string"))?
		);
		let encyclopedia_name = 
			json
				.get("encyclopedia")
				.ok_or(aerr!("world meta does not have encyclopedia"))?
				.as_str()
				.ok_or(aerr!("world meta encyclopedia is not a string"))?
				.to_string();
		Ok(WorldMeta{
			default_room,
			encyclopedia_name
		})
	}
	
	pub fn load_room(&self, id: RoomId) -> Result<RoomTemplate> {
		let fname = id.name.splitn(2, '+').next().unwrap().to_string() + ".json";
		let path = self.directory.join("maps").join(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		let template = RoomTemplate::from_json(&json)?;
		Ok(template)
	}
	
	pub fn load_encyclopedia(&self, name: &str) -> Result<Encyclopedia> {
		let fname: String = name.to_string() + ".json";
		let encyclopedia = Encyclopedia::from_json(
			serde_json::from_str(
				&fs::read_to_string(
					self.directory
						.join("encyclopediae")
						.join(&fname)
				)?
			)?
		)?;
		encyclopedia.validate()?;
		Ok(encyclopedia)
	}
}

pub struct WorldMeta {
	pub encyclopedia_name: String,
	pub default_room: RoomId
}

