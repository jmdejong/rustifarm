

use serde_json::{Value, json};
use specs::Entity;
use crate::{PlayerId, Pos};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
	
	pub fn to_position(self) -> Pos {
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
	Take(Option<usize>),
	Drop(usize),
	Use(usize),
	Attack(Vec<Direction>),
	AttackTarget(Entity),
	Interact(Vec<Direction>, Option<String>),
}


impl Control {
	pub fn from_json(val: &Value) -> Option<Control>{
		if let Value::String(control_type) = val.get(0)? {
			match control_type.as_str() {
				"move" => match Direction::from_json(val.get(1)?) {
					Some(dir) => Some(Control::Move(dir)),
					None => None
				},
				"take" => Some(Control::Take(val.get(1).unwrap_or(&json!(0)).as_u64().map(|idx| idx as usize))),
				"drop" => Some(Control::Drop(val.get(1)?.as_u64().unwrap_or(0) as usize)),
				"use" => Some({
					let arr = val.as_array()?;
					let mut rank = 0;
					if arr.len() == 3 {
						if arr[1].as_str()? != "inventory" {
							return None;
						}
						rank = arr[2].as_u64()?;
					} else if arr.len() == 2 {
						rank = arr[1].as_u64()?;
					} else if arr.len() > 1 {
						return None;
					}
					Control::Use(rank as usize)
				}),
				"attack" => Some(Control::Attack(
					parse_directions(val.get(1)?)?
				)),
				"interact" => Some(Control::Interact(
					parse_directions(val.get(1)?)?,
					if let Some(argument) = val.get(2) {
						match argument {
							Value::String(arg) => Some(arg.to_string()),
							Value::Null => None,
							_ => {return None}
						}
					} else {
						None
					}
				)),
				_ => None
			}
		} else {None}
	}
}

fn parse_directions(val: &Value) -> Option<Vec<Direction>> {
	val.as_array()?.into_iter().map(Direction::from_json).collect()
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

