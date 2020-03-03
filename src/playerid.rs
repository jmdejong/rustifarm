
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub struct PlayerId {
	pub name: String
}

impl PlayerId {
	pub fn to_string(&self) -> String {
		self.name.clone()
	}
}
