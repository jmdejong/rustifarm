
use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct RoomId(pub String);

impl RoomId {
	pub fn format(&self, dict: HashMap<&str, &str>) -> Self {
		let name = dict.into_iter().fold(self.0.clone(), |name, (from, to)| name.replace(from, to));
		Self(name)
	}
}


impl fmt::Display for RoomId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
