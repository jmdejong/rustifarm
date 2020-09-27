
use std::path::{PathBuf};
use std::fs;
use json5;
use serde::{Serialize, Deserialize};
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
		let meta = json5::from_str(&text)?;
		Ok(meta)
	}
	
	pub fn load_room(&self, id: RoomId) -> Result<RoomTemplate> {
		let fname = id.to_string().splitn(2, '+').next().unwrap().to_string() + ".json";
		let path = self.directory.join("maps").join(fname);
		let text = fs::read_to_string(path)?;
		let template = json5::from_str(&text)?;
		Ok(template)
	}
	
	pub fn load_encyclopedia(&self, name: &str) -> Result<Encyclopedia> {
		let fname: String = name.to_string() + ".json";
		let encyclopedia: Encyclopedia = 
			json5::from_str(
				&fs::read_to_string(
					self.directory
						.join("encyclopediae")
						.join(&fname)
				)?
			).map_err(|e|aerr!("failed to load encyclopedia {}: {}", name, e))?;
		encyclopedia.validate()?;
		Ok(encyclopedia)
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct WorldMeta {
	pub encyclopediae: Vec<String>,
	pub default_room: RoomId
}

