

use serde_json::Value;
use super::components::Pos;

#[derive(Debug, Clone)]
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
	
	pub fn to_position(&self) -> Pos {
		match self {
			Direction::North => Pos::new(0, -1),
			Direction::South => Pos::new(0, 1),
			Direction::East => Pos::new(1, 0),
			Direction::West => Pos::new(-1, 0),
			Direction::None => Pos::new(0, 0)
		}
	}
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Action {
	Join(String),
	Leave(String),
	Input(String, Control)
}

