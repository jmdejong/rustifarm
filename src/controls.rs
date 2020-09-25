

use serde::{Serialize, Deserialize, Deserializer, de};
use serde_json::{Value, json};
use specs::Entity;
use crate::{PlayerId, Pos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Direction {
	North,
	South,
	East,
	West,
	#[serde(alias="")]
	None
}

impl Direction {
	
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


impl<'de> Deserialize<'de> for Control {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let val = Value::deserialize(deserializer)?;
		Self::from_json(&val).ok_or(de::Error::custom(format!("invalid control {:?}", val)))
	}
}
impl Control {
	fn from_json(val: &Value) -> Option<Control>{
		if let Value::String(control_type) = val.get(0)? {
			match control_type.as_str() {
				"move" => Direction::deserialize(val.get(1)?).map(|dir| Control::Move(dir)).ok(),
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
	val.as_array()?.into_iter().map(|v|Direction::deserialize(v).ok()).collect()
}

#[derive(Debug, Clone)]
pub enum Action {
	Join(PlayerId),
	Leave(PlayerId),
	Input(PlayerId, Control)
}

