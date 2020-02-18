
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
}

