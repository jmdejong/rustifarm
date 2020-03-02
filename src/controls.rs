

use serde_json::Value;
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
	Take(Option<usize>),
	Drop(usize),
	Use(usize),
	Attack(Vec<Direction>),
	AttackTarget(Entity)
}


impl Control {
	pub fn from_json(val: &Value) -> Option<Control>{
		if let Value::String(control_type) = val.get(0)? {
			match control_type.as_str() {
				"move" => match Direction::from_json(val.get(1)?) {
					Some(dir) => Some(Control::Move(dir)),
					None => None
				},
				"take" => Some(Control::Take(val.get(1)?.as_u64().map(|idx| idx as usize))),
				"drop" => Some(Control::Drop(val.get(1)?.as_u64().unwrap_or(0) as usize)),
				"use" => Some(Control::Use(val.get(1)?.as_u64().unwrap_or(0) as usize)),
				"attack" => Some(Control::Attack({
					let mut directions = Vec::new();
					for dir in val.get(1)?.as_array()? {
						directions.push(Direction::from_json(dir)?);
					}
					directions
				})),
				_ => None
			}
		} else {None}
	}
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

