
use serde_json::{Value, json};
use super::util::ToJson;
// use serde::Serialize;

// #[derive(Serialize)]
pub struct WorldMessage {
	pub updates: Vec<WorldUpdate>
	
}

impl ToJson for WorldMessage {
	fn to_json(&self) -> Value {
		let updates: Vec<Value> = self.updates.iter().map(|u| u.to_json()).collect();
		json!(["world", updates])
	}
}

pub enum WorldUpdate {
	Field(FieldMessage)
}

impl ToJson for WorldUpdate {
	fn to_json(&self) -> Value {
		match self {
			WorldUpdate::Field(msg) => Value::Array(vec![Value::String("field".to_string()), msg.to_json()])
		}
	}
}

pub struct FieldMessage {
	width: i32,
	height: i32,
	field: Vec<usize>,
	mapping: Vec<Vec<String>>
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


