
use std::collections::HashMap;
use serde_json::Value;
use specs::{
	Component,
	HashMapStorage
};
use crate::{
	Sprite
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot {
	Hand,
	Body
}

impl Slot {
	pub fn from_str(txt: &str) -> Option<Self> {
		match txt {
			"hand" => Some(Self::Hand),
			"body" => Some(Self::Body),
			_ => None
		}
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
	Strength,
	Defence,
	Mining
}

impl Stat {
	pub fn from_str(txt: &str) -> Option<Self> {
		match txt {
			"strength" => Some(Self::Strength),
			"defence" => Some(Self::Defence),
			"mining" => Some(Self::Mining),
			_ => None
		}
	}
}


#[derive(Debug, Clone, PartialEq)]
pub struct Equippable {
	pub slot: Slot,
	pub stats: HashMap<Stat, i64>,
	pub sprite: Option<Sprite>
}

impl Equippable {
	pub fn from_json(val: &Value) -> Option<Self> {
		Some(Equippable{
			slot: Slot::from_str(val.get("slot")?.as_str()?)?,
			stats: val
				.get("stats")?
				.as_object()?
				.into_iter()
				.map(|(k, v)| 
					Some((Stat::from_str(k.as_str())?, v.as_i64()?))
				)
				.collect::<Option<HashMap<Stat, i64>>>()?,
			sprite: if let Some(spr) = val.get("sprite") {
					Some(Sprite{name: spr.as_str()?.to_string()})
				} else {None}
		})
	}
}



#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Equipment {
	pub slots: Vec<Slot>
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use serde_json::json;
	
	
	#[test]
	fn slots() {
		assert_eq!(Slot::from_str("hand"), Some(Slot::Hand));
		assert_eq!(Slot::from_str("body"), Some(Slot::Body));
		assert_eq!(Slot::from_str("hands"), None);
		assert_eq!(Slot::from_str("head"), None);
	}
	
	#[test]
	fn stats() {
		assert_eq!(Stat::from_str("strength"), Some(Stat::Strength));
		assert_eq!(Stat::from_str("defence"), Some(Stat::Defence));
		assert_eq!(Stat::from_str("hand"), None);
		assert_eq!(Stat::from_str("body"), None);
		assert_eq!(Stat::from_str("attack"), None);
	}
	
	#[test]
	fn equippable_from_json() {
		assert_eq!(
			Equippable::from_json(&json!({"slot": "hand", "stats": {"strength": 10}})),
			Some(Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10), sprite: None})
		);
	}
	
}
