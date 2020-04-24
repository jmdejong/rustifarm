
use std::path::{PathBuf};
use std::fs;
use std::env;
use std::io::ErrorKind;

use serde_json;
use serde::{Serialize, Deserialize};
use ring::digest;
use base64;

use crate::{
	PlayerId,
	errors::AnyError,
	util::write_file_safe
};


pub enum LoaderError {
	MissingResource(AnyError),
	InvalidResource(AnyError)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
	Player,
	Bridge
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
	pub name: String,
	pub pass_token: String,
	pub salt: String,
	pub role: UserRole
}

impl User {
	pub fn validate_token(&self, token: &str) -> bool {
		if let (Ok(saved), Ok(mut given), Ok(mut salt)) = (base64::decode(&self.pass_token), base64::decode(token), base64::decode(&self.salt)) {
			given.append(&mut salt);
			let hashed: Vec<u8> = digest::digest(&digest::SHA256, &given).as_ref().to_vec();
			hashed == saved
		} else {
			false
		}
	}
}

macro_rules! inv {
	($code:expr) => {($code).map_err(|err| LoaderError::InvalidResource(Box::new(err)))}
}


pub trait UserRegistry {
	
	fn load_user(&self, id: &PlayerId) -> Result<User, LoaderError>;
	
	fn register_user(&self, id: &PlayerId, user: &User) -> Result<(), AnyError>;
	
	fn user_exists(&self, id: &PlayerId) -> bool {
		match self.load_user(id) {
			Ok(_) => true,
			Err(LoaderError::InvalidResource(_)) => true,
			Err(LoaderError::MissingResource(_)) => false
		}
	}
}


pub struct FileRegister {
	directory: PathBuf
}

impl FileRegister {
	pub fn new(path: PathBuf) -> Self {
		Self {
			directory: path
		}
	}
	
	pub fn default_register_dir() -> Option<PathBuf> {
		if let Some(pathname) = env::var_os("XDG_DATA_HOME") {
			let mut path = PathBuf::from(pathname);
			path.push("asciifarm");
			path.push("users");
			Some(path)
		} else if let Some(pathname) = env::var_os("HOME") {
			let mut path = PathBuf::from(pathname);
			path.push(".asciifarm");
			path.push("users");
			Some(path)
		} else {
			None
		}
	}
}

impl UserRegistry for FileRegister {
	
	fn load_user(&self, id: &PlayerId) -> Result<User, LoaderError> {
		let mut path = self.directory.clone();
		let fname = id.to_string() + ".auth.json";
		path.push(fname);
		let text = fs::read_to_string(path).map_err(|err| {
			if err.kind() == ErrorKind::NotFound {
				LoaderError::MissingResource(Box::new(err))
			} else {
				LoaderError::InvalidResource(Box::new(err))
			}
		})?;
		let user: User = inv!(serde_json::from_str(&text))?;
		Ok(user)
	}
	
	fn register_user(&self, id: &PlayerId, user: &User) -> Result<(), AnyError> {
		let mut path = self.directory.clone();
		fs::create_dir_all(&path)?;
		let fname = id.to_string() + ".auth.json";
		path.push(fname);
		let text = serde_json::to_string(user)?;
		write_file_safe(path, text)?;
		Ok(())
	}
}



