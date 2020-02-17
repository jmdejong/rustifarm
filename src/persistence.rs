
use std::path::PathBuf;
use std::fs;
use serde_json;
use serde_json::Value;
use crate::{
	PlayerId,
	savestate::SaveState,
	playerstate::PlayerState,
	util::Result,
	aerr
};

pub trait PersistentStorage {
	
	fn load_room(&self, name: String) -> Result<SaveState>;
	
	fn load_player(&self, id: PlayerId) -> Result<PlayerState>;
	
	fn save_room(&self, name: String, state: SaveState) -> Result<()>;
	
	fn save_player(&self, id: PlayerId, sate: PlayerState) -> Result<()>;
	
}


pub struct FileStorage {
	directory: PathBuf
}

impl PersistentStorage for FileStorage {
	
	fn load_room(&self, name: String) -> Result<SaveState> {
		let mut path = self.directory.clone();
		path.push("rooms");
		let fname = name + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		SaveState::from_json(&json).ok_or(aerr!("not a valid save state"))
	}
	
	fn load_player(&self, id: PlayerId) -> Result<PlayerState> {
		let mut path = self.directory.clone();
		path.push("players");
		let fname = id.name + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		PlayerState::from_json(&json).ok_or(aerr!("not a valid save state"))
	}
	
	fn save_room(&self, name: String, state: SaveState) -> Result<()> {
		let mut path = self.directory.clone();
		path.push("rooms");
		fs::create_dir_all(&path)?;
		let fname = name + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		// todo: write to a temp file first
		fs::write(path, text)?;
		Ok(())
	}
	
	fn save_player(&self, id: PlayerId, state: PlayerState) -> Result<()> {
		let mut path = self.directory.clone();
		path.push("players");
		fs::create_dir_all(&path)?;
		let fname = id.name + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		// todo: write to a temp file first
		fs::write(path, text)?;
		Ok(())
	}
}

