

use std::collections::HashSet;
use serde;
use serde::{Deserialize, Serialize};
use crate::{
	Template,
	components::{
		Flag,
		equipment::Equippable
	}
};



#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);

#[derive(Debug, Clone)]
pub struct Item {
	pub ent: Template,
	pub name: String,
	pub action: ItemAction
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemAction {
	Eat(i64),
	Build(Template, HashSet<Flag>, HashSet<Flag>),
	Equip(Equippable),
	None
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use crate::components::equipment::*;
	use serde_json::json;
	
	#[test]
	fn equip_deserialise() {
		assert_eq!(
			ItemAction::deserialize(&json!({"equip": {"slot": "hand", "stats": {"strength": 10}}})).unwrap(),
			ItemAction::Equip(Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10), sprite: Option::None})
		);
	}
	#[test]
	fn invalid_stat() {
		assert_eq!(
			ItemAction::deserialize(&json!({"equip": {"slot": "hand", "stats": {"attack": 50}}})).ok(),
			Option::None
		);
	}
}

