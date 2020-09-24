
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use specs::{
	Component,
	HashMapStorage
};
use crate::{
	Sprite
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
pub enum Slot {
	Hand,
	Body,
	Back
}

impl Slot {
	pub fn from_str(txt: &str) -> Option<Self> {
		match txt {
			"hand" => Some(Self::Hand),
			"body" => Some(Self::Body),
			"back" => Some(Self::Back),
			_ => None
		}
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")] 
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


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equippable {
	pub slot: Slot,
	pub stats: HashMap<Stat, i64>,
	pub sprite: Option<Sprite>
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
	fn equippable_deserialize() {
		assert_eq!(
			Equippable::deserialize(&json!({"slot": "hand", "stats": {"strength": 10}})).unwrap(),
			Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10), sprite: None}
		);
	}
	
}
