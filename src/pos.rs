

use std::ops::Add;
use serde_json::{Value, json};
use serde::{Serialize, Serializer, ser::SerializeTuple};
use super::util::{clamp, ToJson};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos {
	pub x: i32,
	pub y: i32
}


impl Pos {
	
	pub fn new(x: i32, y: i32) -> Pos {
		Pos {x, y}
	}
	
	pub fn clamp(self, smaller: Pos, larger: Pos) -> Pos {
		Pos {
			x: clamp(self.x, smaller.x, larger.x),
			y: clamp(self.y, smaller.y, larger.y)
		}
	}
}


impl Serialize for Pos {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut tup = serializer.serialize_tuple(2)?;
		tup.serialize_element(&self.x)?;
		tup.serialize_element(&self.y)?;
		tup.end()
	}
}

impl Add<Pos> for Pos {
	type Output = Pos;

	fn add(self, other: Pos) -> Pos {
		Pos {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl ToJson for Pos {
	fn to_json(&self) -> Value {
		json!([self.x, self.y])
	}
}
