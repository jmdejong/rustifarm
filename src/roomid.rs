
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RoomId {
	pub name: String
}

impl RoomId {
	pub fn from_str(name: &str) -> Self {
		Self {name: name.to_string()}
	}
	pub fn to_string(&self) -> String {
		self.name.clone()
	}
	pub fn format(&self, dict: HashMap<&str, &str>) -> Self {
		let name = dict.into_iter().fold(self.name.clone(), |name, (from, to)| name.replace(from, to));
		Self {name}
	}
}

