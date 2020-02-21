
use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct Sprite {
	pub name: String
}

impl Serialize for Sprite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.name.as_str())
    }
}
