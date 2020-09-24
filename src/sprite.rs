
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Sprite {
	pub name: String
}

impl Serialize for Sprite {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		serializer.serialize_str(self.name.as_str())
	}
}
impl<'de> Deserialize<'de> for Sprite {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		Ok(Self{name: String::deserialize(deserializer)?})
	}
}
