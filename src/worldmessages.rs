
use serde_json::{Value, json};
use serde::Serialize;
use crate::util::ToJson;
use crate::{Pos, Sprite};


#[derive(Debug, Clone)]
pub struct WorldMessage {
	pub updates: Vec<WorldUpdate>
}

impl ToJson for WorldMessage {
	fn to_json(&self) -> Value {
		let updates: Vec<Value> = self.updates.iter().map(|u| u.to_json()).collect();
		json!(["world", updates])
	}
}

#[derive(Debug, Clone)]
pub enum WorldUpdate {
	Field(FieldMessage),
	Pos(Pos),
	Change(Vec<(Pos, Vec<Sprite>)>),
	Inventory(Vec<String>),
	Health(i64, i64)
}

impl ToJson for WorldUpdate {
	fn to_json(&self) -> Value {
		match self {
			WorldUpdate::Field(msg) => json!(["field", msg]),
			WorldUpdate::Pos(pos) => json!(["playerpos", pos]),
			WorldUpdate::Change(changes) => json!(["changecells", changes]),
			WorldUpdate::Inventory(items) => json!(["inventory", items]),
			WorldUpdate::Health(health, maxhealth) => json!(["health", [health, maxhealth]])
		}
	}
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldMessage {
	pub width: i64,
	pub height: i64,
	pub field: Vec<usize>,
	pub mapping: Vec<Vec<Sprite>>
}



