

use std::collections::HashMap;
use serde_json::{json, Value};
use crate::{
	parameter::Parameter,
	Result,
	aerr
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EntityType(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
	pub name: EntityType,
	pub args: Vec<Parameter>,
	pub kwargs: HashMap<String, Parameter>,
	pub save: bool
}


impl Template {
	
	pub fn new(name: &str, kwargs: HashMap<String, Parameter>) -> Self {
		Self {
			name: EntityType(name.to_string()),
			args: Vec::new(),
			kwargs,
			save: true
		}
	}
	
	pub fn unsaved(mut self) -> Self {
		self.save = false;
		self
	}
	
	pub fn empty(name: &str) -> Self {
		Self::new(name, HashMap::new())
	}
	
	pub fn from_json(val: &Value) -> Result<Template> {
		if val.is_string(){
			return Ok(Self::empty(val.as_str().unwrap()));
		}
		let name = EntityType(val.get("type").ok_or(aerr!("template doesn't have 'type'"))?.as_str().ok_or(aerr!("template type not a string"))?.to_string());
		let mut args = Vec::new();
		for arg in val.get("args").unwrap_or(&json!([])).as_array().ok_or(aerr!("template args not an array"))? {
			args.push(Parameter::guess_from_json(arg).ok_or(aerr!("template arg not a parameter"))?);
		}
		let mut kwargs = HashMap::new();
		for (key, arg) in val.get("kwargs").unwrap_or(&json!({})).as_object().ok_or(aerr!("template kwargs not a json object"))? {
			kwargs.insert(key.to_string(), Parameter::guess_from_json(arg).ok_or(aerr!("template arg not a parameter"))?);
		}
		let save = val.get("save").unwrap_or(&json!(true)).as_bool().ok_or(aerr!("save not a bool"))?;
		Ok(Template {name, args, kwargs, save})
	}
	
	pub fn to_json(&self) -> Value {
		if self.args.is_empty() && self.kwargs.is_empty() {
			return json!(self.name.0);
		}
		let jsonargs: Vec<Value> = self.args.iter().map(|a| a.to_json()).collect();
		let jsonkwargs: HashMap<&String, Value> = self.kwargs.iter().map(|(k, a)| (k, a.to_json())).collect();
		json!({
			"type": self.name.0,
			"args": jsonargs,
			"kwargs": jsonkwargs
		})
	}
}
