
use specs::{Component, DenseVecStorage};
use crate::{Template};

use super::equipment::Equippable;

#[derive(Component, Debug, Clone)]
pub struct Item {
	pub ent: Template,
	pub name: String,
	pub action: ItemAction
}



use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum ItemAction {
	Eat(i64),
	Build(Template),
	Equip(Equippable),
	None
}

use ItemAction::{Eat, Build, Equip, None};

impl ItemAction {
	pub fn to_json(&self) -> Value {
		match self {
			Eat(health) => json!(["eat", health]),
			Build(template) => json!(["build", template.to_json()]),
			Equip(equippable) => json!(["equip", equippable.to_json()]),
			None => json!(["none", null])
		}
	}
	
	pub fn from_json(val: &Value) -> Option<Self> {
		let typ = val.get(0)?;
		let arg = val.get(1)?;
		Some(match typ.as_str()? {
			"eat" => Eat(arg.as_i64()?),
			"build" => Build(Template::from_json(arg).ok()?),
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
	use super::super::equipment::*;
	
	#[test]
	fn equip_from_json() {
		assert_eq!(
			ItemAction::from_json(&json!(["equip", {"slot": "hand", "stats": {"strength": 10}}])),
			Some(ItemAction::Equip(Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10)}))
		);
		assert_eq!(
			ItemAction::from_json(&json!(["equip", {"slot": "hand", "stats": {"attack": 50}}])),
			Option::None
		);
	}
}

