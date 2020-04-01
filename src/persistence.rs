
use std::path::{PathBuf, Path};
use std::fs;
use std::env;
use serde_json;
use serde_json::{Value, json};
use crate::{
	PlayerId,
	RoomId,
	savestate::SaveState,
	playerstate::PlayerState,
	Timestamp,
	Result,
	aerr
};

pub trait PersistentStorage {
	
	fn load_room(&self, id: RoomId) -> Result<SaveState>;
	fn load_player(&self, id: PlayerId) -> Result<PlayerState>;
	fn load_world_meta(&self) -> Result<Timestamp>;
	
	fn save_room(&self, id: RoomId, state: SaveState) -> Result<()>;
	fn save_player(&self, id: PlayerId, sate: PlayerState) -> Result<()>;
	fn save_world_meta(&self, time: Timestamp) -> Result<()>;
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
	
	pub fn savedir() -> Option<PathBuf> {
		if let Some(pathname) = env::var_os("ASCIIFARM_SAVE_DIR") {
			Some(PathBuf::from(pathname))
		} else if let Some(pathname) = env::var_os("XDG_DATA_HOME") {
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
	
	fn load_room(&self, id: RoomId) -> Result<SaveState> {
		let mut path = self.directory.clone();
		path.push("rooms");
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		SaveState::from_json(&json)
	}
	
	fn load_player(&self, id: PlayerId) -> Result<PlayerState> {
		let mut path = self.directory.clone();
		path.push("players");
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		PlayerState::from_json(&json)
	}
	
	fn load_world_meta(&self) -> Result<Timestamp> {
		let mut path = self.directory.clone();
		path.push("world.save.json");
		let text = fs::read_to_string(path)?;
		let json: Value = serde_json::from_str(&text)?;
		Ok(
			Timestamp(
				json
					.get("steps").ok_or(aerr!("world data does not have steps"))?
					.as_i64().ok_or(aerr!("timestamp not an int"))?
			)
		)
	}
	
	fn save_room(&self, id: RoomId, state: SaveState) -> Result<()> {
		let mut path = self.directory.clone();
		path.push("rooms");
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		write_file_safe(path, text)?;
		Ok(())
	}
	
	fn save_player(&self, id: PlayerId, state: PlayerState) -> Result<()> {
		let mut path = self.directory.clone();
		path.push("players");
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".save.json";
		path.push(fname);
		let text = state.to_json().to_string();
		write_file_safe(path, text)?;
		Ok(())
	}
	
	fn save_world_meta(&self, time: Timestamp) -> Result<()> {
		let mut path = self.directory.clone();
		fs::create_dir_all(&path)?;
		path.push("world.save.json");
		write_file_safe(path, json!({"steps": time.0}).to_string())?;
		Ok(())
	}
}

fn write_file_safe<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
	let temppath = path
		.as_ref()
		.with_file_name(
			format!(
				"tempfile_{}_{}.tmp",
				path.as_ref().file_name().ok_or(aerr!("writing to directory"))?.to_str().unwrap_or("invalid"),
				rand::random::<u64>()
			)
		);
	fs::write(&temppath, contents)?;
	fs::rename(&temppath, path)?;
	Ok(())
}

