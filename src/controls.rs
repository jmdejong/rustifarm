

use serde_json::Value;
use super::components::Position;

#[derive(Debug)]
pub enum Direction {
	North,
	South,
	East,
	West,
	None
}

impl Direction {
	fn from_json(val: &Value) -> Option<Direction>{
		match val {
			Value::String(txt) => match txt.as_str() {
				"north" => Some(Direction::North),
				"south" => Some(Direction::South),
				"east" => Some(Direction::East),
				"west"=> Some(Direction::West),
				"" => Some(Direction::None),
				_ => None
			}
			Value::Null => Some(Direction::None),
			_ => None
		}
	}
	
	pub fn to_position(&self) -> Position {
		match self {
			Direction::North => Position::new(0, -1),
			Direction::South => Position::new(0, 1),
			Direction::East => Position::new(1, 0),
			Direction::West => Position::new(-1, 0),
			Direction::None => Position::new(0, 0)
		}
	}
}

#[derive(Debug)]
pub enum Control {
	Move(Direction),
	Take(u64)
}


impl Control {
	pub fn from_json(val: Value) -> Option<Control>{
		if let Value::String(control_type) = &val[0] {
			match control_type.as_str() {
				"move" => match Direction::from_json(&val[1]) {
					Some(dir) => Some(Control::Move(dir)),
					None => None
				},
				"take" => match val[1].as_u64() {
					Some(rank) => Some(Control::Take(rank)),
					_ => None
				}
				_ => None
			}
		} else {None}
	}
}
