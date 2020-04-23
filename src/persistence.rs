
use std::path::{PathBuf};
use std::fs;
use std::env;
use std::io::ErrorKind;
use serde_json;
use serde_json::{Value, json};
use crate::{
	PlayerId,
	RoomId,
	savestate::SaveState,
	playerstate::PlayerState,
	Timestamp,
	aerr,
	errors::AnyError,
	util::write_file_safe
};


pub enum LoaderError {
	MissingResource(AnyError),
	InvalidResource(AnyError)
}

macro_rules! inv {
	($code:expr) => {($code).map_err(|err| LoaderError::InvalidResource(Box::new(err)))}
}


pub trait PersistentStorage {
	
	fn load_room(&self, id: RoomId) -> Result<SaveState, LoaderError>;
	fn load_player(&self, id: PlayerId) -> Result<PlayerState, LoaderError>;
	fn load_world_meta(&self) -> Result<Timestamp, LoaderError>;
	
	fn save_room(&self, id: RoomId, state: SaveState) -> Result<(), AnyError>;
	fn save_player(&self, id: PlayerId, sate: PlayerState) -> Result<(), AnyError>;
	fn save_world_meta(&self, time: Timestamp) -> Result<(), AnyError>;
}


pub struct FileStorage {
	directory: PathBuf
}

impl FileStorage {
	pub fn new(path: PathBuf) -> Self {
		Self {
			directory: path
		}
	}
	
	pub fn default_save_dir() -> Option<PathBuf> {
		if let Some(pathname) = env::var_os("XDG_DATA_HOME") {
			let mut path = PathBuf::from(pathname);
			path.push("asciifarm");
			path.push("saves");
			Some(path)
		} else if let Some(pathname) = env::var_os("HOME") {
			let mut path = PathBuf::from(pathname);
			path.push(".asciifarm");
			path.push("saves");
			Some(path)
		} else {
			None
		}
	}
}

impl PersistentStorage for FileStorage {
	
	fn load_room(&self, id: RoomId) -> Result<SaveState, LoaderError> {
		let mut path = self.directory.clone();
		path.push("rooms");
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path).map_err(|err| {
			if err.kind() == ErrorKind::NotFound {
				LoaderError::MissingResource(Box::new(err))
			} else {
				LoaderError::InvalidResource(Box::new(err))
			}
		})?;
		let json: Value = inv!(serde_json::from_str(&text))?;
		let state = inv!(SaveState::from_json(&json))?;
		Ok(state)
	}
	
	fn load_player(&self, id: PlayerId) -> Result<PlayerState, LoaderError> {
		let mut path = self.directory.clone();
		path.push("players");
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path).map_err(|err| {
			if err.kind() == ErrorKind::NotFound {
				LoaderError::MissingResource(Box::new(err))
			} else {
				LoaderError::InvalidResource(Box::new(err))
			}
		})?;
		let json: Value = inv!(serde_json::from_str(&text))?;
		let state = inv!(PlayerState::from_json(&json))?;
		Ok(state)
	}
	
	fn load_world_meta(&self) -> Result<Timestamp, LoaderError> {
		let mut path = self.directory.clone();
		path.push("world.save.json");
		let text = fs::read_to_string(path).map_err(|err| {
			if err.kind() == ErrorKind::NotFound {
				LoaderError::MissingResource(Box::new(err))
			} else {
				LoaderError::InvalidResource(Box::new(err))
			}
		})?;
		let json: Value = inv!(serde_json::from_str(&text))?;
		Ok(
			Timestamp(
				inv!(inv!(json
					.get("steps").ok_or(aerr!("world data does not have steps")))?
					.as_i64().ok_or(aerr!("timestamp not an int")))?
			)
		)
	}
	
	fn save_room(&self, id: RoomId, state: SaveState) -> Result<(), AnyError> {
		let mut path = self.directory.clone();
		path.push("rooms");
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		write_file_safe(path, text)?;
		Ok(())
	}
	
	fn save_player(&self, id: PlayerId, state: PlayerState) -> Result<(), AnyError> {
		let mut path = self.directory.clone();
		path.push("players");
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		write_file_safe(path, text)?;
		Ok(())
	}
	
	fn save_world_meta(&self, time: Timestamp) -> Result<(), AnyError> {
		let mut path = self.directory.clone();
		fs::create_dir_all(&path)?;
		path.push("world.save.json");
		write_file_safe(path, json!({"steps": time.0}).to_string())?;
		Ok(())
	}
}


