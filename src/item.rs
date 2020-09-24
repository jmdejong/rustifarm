

use std::collections::HashSet;
use std::str::FromStr;
use serde;
use serde::Deserialize;
use serde_json::{Value};
use crate::{
	Template,
	components::{
		Flag,
		equipment::Equippable
	},
	errors::{ParseError}
};



#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct ItemId(pub String);

impl FromStr for ItemId {
	type Err = ParseError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(s.to_string()))
	}
}

#[derive(Debug, Clone)]
pub struct Item {
	pub ent: Template,
	pub name: String,
	pub action: ItemAction
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemAction {
	Eat(i64),
	Build(Template, HashSet<Flag>, HashSet<Flag>),
	Equip(Equippable),
	None
}

use ItemAction::{Eat, Build, Equip, None};

impl ItemAction {
	
	pub fn from_json(val: &Value) -> Option<Self> {
		let typ = val.get(0)?;
		let arg = val.get(1)?;
		Some(match typ.as_str()? {
			"eat" => Eat(arg.as_i64()?),
			"build" => Build(
				Template::from_json(arg.get(0)?).ok()?,
				arg.get(1)?.as_array()?.iter().map(|v| Flag::from_str(v.as_str()?).ok()).collect::<Option<HashSet<Flag>>>()?,
				arg.get(2)?.as_array()?.iter().map(|v| Flag::from_str(v.as_str()?).ok()).collect::<Option<HashSet<Flag>>>()?
			),
			"none" => None,
			"equip" => Equip(Equippable::from_json(arg)?),
			_ => {return Option::None}
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use crate::components::equipment::*;
	use serde_json::json;
	
	#[test]
	fn equip_from_json() {
		assert_eq!(
			ItemAction::from_json(&json!(["equip", {"slot": "hand", "stats": {"strength": 10}}])),
			Some(ItemAction::Equip(Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10), sprite: Option::None}))
		);
		assert_eq!(
			ItemAction::from_json(&json!(["equip", {"slot": "hand", "stats": {"attack": 50}}])),
			Option::None
		);
	}
}

