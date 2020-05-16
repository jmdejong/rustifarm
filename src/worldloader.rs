
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
		let encyclopediae = 
			json
				.get("encyclopediae")
				.ok_or(aerr!("world meta does not have encyclopediae"))?
				.as_array()
				.ok_or(aerr!("world meta encyclopediae is not a list"))?
				.iter()
				.map(|v| Ok(v
					.as_str()
					.ok_or(aerr!("world meta encyclopediae item {:?} is not a string", v))?
					.to_string()
				))
				.collect::<Result<Vec<String>>>()?;
		Ok(WorldMeta{
			default_room,
			encyclopediae
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
	pub encyclopediae: Vec<String>,
	pub default_room: RoomId
}

