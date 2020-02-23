
use std::collections::HashMap;
use serde_json::{json, Value};
use crate::{
	Pos,
	Template,
	Result,
	aerr
};

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
	
	pub fn from_json(val: &Value) -> Result<Self> {
		let mut changes = HashMap::new();
		for v in val.get("changes").ok_or(aerr!("save does not have changes"))?.as_array().ok_or(aerr!("changes not an array"))? {
			let pos = Pos::from_json(v.get(0).ok_or(aerr!("change does not have index 0"))?).ok_or(aerr!("change index 0 is not a pos"))?;
			let mut templates = Vec::new();
			for t in v.get(1).ok_or(aerr!("change does not have index 1"))?.as_array().ok_or(aerr!("change index 1 not an array"))? {
				templates.push(Template::from_json(t)?);
			}
			changes.insert(pos, templates);
		}
		Ok(Self {changes})
	}
}
