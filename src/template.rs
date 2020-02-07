

use std::collections::HashMap;
use serde_json::{json, Value};
use crate::parameter::Parameter;

#[derive(Debug, Clone)]
pub struct Template {
	pub name: String,
	pub args: Vec<Parameter>,
	pub kwargs: HashMap<String, Parameter>
}


impl Template {
	
	pub fn new(name: &str, kwargs: HashMap<String, Parameter>) -> Self {
		Self {
			name: name.to_string(),
			args: Vec::new(),
			kwargs
		}
	}
	
	pub fn empty(name: &str) -> Self {
		Self::new(name, HashMap::new())
	}
	
	pub fn from_json(val: &Value) -> Option<Template> {
		if val.is_string(){
			return Some(Self::empty(val.as_str()?));
		}
		let name = val.get("type")?.as_str()?.to_string();
		let mut args = Vec::new();
		for arg in val.get("args").unwrap_or(&json!({})).as_array()? {
			args.push(Parameter::guess_from_json(arg)?);
		}
		let mut kwargs = HashMap::new();
		for (key, arg) in val.get("kwargs").unwrap_or(&json!({})).as_object()? {
			kwargs.insert(key.to_string(), Parameter::guess_from_json(arg)?);
		}
		Some(Template {name, args, kwargs})
	}
}
