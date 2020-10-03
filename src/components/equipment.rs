
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use crate::{
	Sprite
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Slot {
	Hand,
	Body,
	Back
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Stat {
	Strength,
	Defence,
	Mining,
	Gathering
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equippable {
	pub slot: Slot,
	pub stats: HashMap<Stat, i64>,
	pub sprite: Option<Sprite>
}




#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use serde_json::json;
	use std::str::FromStr;
	
	
	#[test]
	fn slots() {
		assert_eq!(Slot::from_str("hand"), Ok(Slot::Hand));
		assert_eq!(Slot::from_str("body"), Ok(Slot::Body));
		assert!(Slot::from_str("hands").is_err());
		assert!(Slot::from_str("head").is_err());
	}
	
	#[test]
	fn stats() {
		assert_eq!(Stat::from_str("strength"), Ok(Stat::Strength));
		assert_eq!(Stat::from_str("defence"), Ok(Stat::Defence));
		assert!(Stat::from_str("hand").is_err());
		assert!(Stat::from_str("body").is_err());
		assert!(Stat::from_str("attack").is_err());
	}
	
	#[test]
	fn equippable_deserialize() {
		assert_eq!(
			Equippable::deserialize(&json!({"slot": "hand", "stats": {"strength": 10}})).unwrap(),
			Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10), sprite: None}
		);
	}
	
}
