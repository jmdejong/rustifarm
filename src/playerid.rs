
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub struct PlayerId {
	pub name: String
}

impl PlayerId {
	pub fn from_str(name: &str) -> Self {
		Self {name: name.to_string()}
	}
	pub fn to_string(&self) -> String {
		self.name.clone()
	}
}
