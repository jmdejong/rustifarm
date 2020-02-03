
use serde_json::{Value, json};
use super::util::ToJson;
use super::pos::Pos;

#[derive(Clone)]
pub struct WorldMessage {
	pub updates: Vec<WorldUpdate>
}

impl ToJson for WorldMessage {
	fn to_json(&self) -> Value {
		let updates: Vec<Value> = self.updates.iter().map(|u| u.to_json()).collect();
		json!(["world", updates])
	}
}

#[derive(Clone)]
pub enum WorldUpdate {
	Field(FieldMessage),
	Pos(Pos)
}

impl ToJson for WorldUpdate {
	fn to_json(&self) -> Value {
		match self {
			WorldUpdate::Field(msg) => Value::Array(vec![Value::String("field".to_string()), msg.to_json()]),
			WorldUpdate::Pos(pos) => Value::Array(vec![Value::String("playerpos".to_string()), pos.to_json()])
		}
	}
}

#[derive(Clone)]
pub struct FieldMessage {
	pub width: i32,
	pub height: i32,
	pub field: Vec<usize>,
	pub mapping: Vec<Vec<String>>
}
impl ToJson for FieldMessage {
	fn to_json(&self) -> Value {
		json!({
			"width": self.width,
			"height": self.height,
			"field": self.field,
			"mapping": self.mapping
		})
	}
}



