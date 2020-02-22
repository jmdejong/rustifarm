
use specs::{Component, DenseVecStorage};
use crate::{Template};

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
	None
}

use ItemAction::{Eat, Build, None};

impl ItemAction {
	pub fn to_json(&self) -> Value {
		match self {
			Eat(health) => json!(["eat", health]),
			Build(template) => json!(["build", template.to_json()]),
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
			_ => {return Option::None}
		})
	}
}
