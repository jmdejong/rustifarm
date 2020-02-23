

use std::ops::Add;
use serde_json::Value;
use serde::{Serialize, Serializer, ser::SerializeTuple};
use crate::util::clamp;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Pos {
	pub x: i64,
	pub y: i64
}


impl Pos {
	
	pub fn new(x: i64, y: i64) -> Pos {
		Pos {x, y}
	}
	
	pub fn clamp(self, smaller: Pos, larger: Pos) -> Pos {
		Pos {
			x: clamp(self.x, smaller.x, larger.x),
			y: clamp(self.y, smaller.y, larger.y)
		}
	}
	
	pub fn from_json(val: &Value) -> Option<Self>{
		Some(Pos {
			x: val.get(0)?.as_i64()?,
			y: val.get(1)?.as_i64()?
		})
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

