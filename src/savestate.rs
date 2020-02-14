
use std::collections::HashMap;
use serde_json::{json, Value};
use crate::Pos;
use crate::template::Template;

pub struct SaveState {
	pub changes: HashMap<Pos, Vec<Template>>
}

impl SaveState {
	
	pub fn new() -> Self {
		Self {
			changes: HashMap::new()
		}
	}
	
	pub fn to_json(&self) -> Value {
		json!({
			"changes": self.changes.iter().map(
					|(pos, templates)|
					(pos, templates.iter().map(|t| t.to_json()).collect())
				).collect::<Vec<(&Pos, Vec<Value>)>>()
		})
	}
	
	pub fn from_json(val: &Value) -> Option<Self> {
		let mut changes = HashMap::new();
		for v in val.get("changes")?.as_array()? {
			let pos = Pos::from_json(v.get(0)?)?;
			let mut templates = Vec::new();
			for t in v.get(1)?.as_array()? {
				templates.push(Template::from_json(t)?);
			}
			changes.insert(pos, templates);
		}
		Some(Self {changes})
	}
}
