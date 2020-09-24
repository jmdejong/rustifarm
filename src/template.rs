

use std::collections::HashMap;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};

use crate::{
	parameter::Parameter,
	PResult,
	perr
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct EntityType(pub String);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from="Option<bool>", into="Option<bool>")]
pub enum SaveOption {
	Default,
	False,
	Always
}

impl From<Option<bool>> for SaveOption {
	fn from(b: Option<bool>) -> Self {
		match b {
			Some(true) => Self::Always,
			Some(false) => Self::False,
			None => Self::Default
		}
	}
}
impl Into<Option<bool>> for SaveOption {
	fn into(self) -> Option<bool> {
		match self {
			Self::Always => Some(true),
			Self::False => Some(false),
			Self::Default => None
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum TemplateSave {
	Name(EntityType),
	Full{
		#[serde(rename = "type")]
		name: EntityType,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		args: Vec<Parameter>,
		#[serde(default, skip_serializing_if = "HashMap::is_empty")]
		kwargs: HashMap<String, Parameter>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		save: Option<bool>
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from="TemplateSave", into="TemplateSave")]
pub struct Template {
	pub name: EntityType,
	pub args: Vec<Parameter>,
	pub kwargs: HashMap<String, Parameter>,
	pub save: SaveOption,
}


impl From<TemplateSave> for Template {
	fn from(ts: TemplateSave) -> Self {
		match ts {
			TemplateSave::Name(name) => Self{name, args: Vec::new(), kwargs: HashMap::new(), save: SaveOption::Default},
			TemplateSave::Full{name, args, kwargs, save} => Self{name, args, kwargs, save: match save {
				Some(true) => SaveOption::Always,
				Some(false) => SaveOption::False,
				None => SaveOption::Default
			}}
		}
	}
}
impl Into<TemplateSave> for Template {
	fn into(self) -> TemplateSave {
		if self.args.is_empty() && self.kwargs.is_empty() && self.save == SaveOption::Default {
			return TemplateSave::Name(self.name);
		}
		TemplateSave::Full {
			name: self.name,
			args: self.args,
			kwargs: self.kwargs,
			save: match self.save {
				SaveOption::Always => Some(true),
				SaveOption::False => Some(false),
				SaveOption::Default => None
			}
		}
	}
}

impl Template {
	
	pub fn new(name: &str, kwargs: HashMap<String, Parameter>) -> Self {
		Self {
			name: EntityType(name.to_string()),
			args: Vec::new(),
			kwargs,
			save: SaveOption::Default
		}
	}
	
	pub fn empty(name: &str) -> Self {
		Self::new(name, HashMap::new())
	}
	
	pub fn should_save(&self) -> bool {
		match self.save {
			SaveOption::Default | SaveOption::Always => true,
			SaveOption::False => false
		}
	}
	
	pub fn from_entity_type(typ: EntityType) -> Self {
		Self {
			name: typ,
			args: Vec::new(),
			kwargs: HashMap::new(),
			save: SaveOption::Default
		}
	}
	
	pub fn unsaved(mut self) -> Self {
		if self.save == SaveOption::Default {
			self.save = SaveOption::False
		}
		self
	}
	
	pub fn merge(mut self, other: Template) -> Self {
		if self.save == SaveOption::Default {
			self.save = other.save;
		}
		for (key, value) in other.kwargs {
			self.kwargs.entry(key).or_insert(value);
		}
		self
	}
	
	pub fn from_json(v: &Value) -> PResult<Template> {
		let val = match v {
			Value::String(s) => json!({"type": s}),
			Value::Array(_) => json!({
				"type": v.get(0).ok_or(perr!("index 0 not in template array {:?}", v))?,
				"kwargs": v.get(1).ok_or(perr!("index 1 not in template array {:?}", v))?
			}),
			Value::Object(_) => v.clone(),
			_ => return Err(perr!("invalid template {:?}", v))
		};
			
		let name = EntityType(val.get("type").ok_or(perr!("template doesn't have 'type'"))?.as_str().ok_or(perr!("template type not a string"))?.to_string());
		let mut args = Vec::new();
		for arg in val.get("args").unwrap_or(&json!([])).as_array().ok_or(perr!("template args not an array"))? {
			args.push(Parameter::guess_from_json(arg)?);
		}
		let mut kwargs = HashMap::new();
		for (key, arg) in val.get("kwargs").unwrap_or(&json!({})).as_object().ok_or(perr!("template kwargs not a json object"))? {
			kwargs.insert(key.to_string(), Parameter::guess_from_json(arg)?);
		}
		let save = 
			if let Some(saveval) = val.get("save") {
				if saveval.as_bool().ok_or(perr!("save not a bool"))? {
					SaveOption::Always
				} else {
					SaveOption::False
				}
			} else {
				SaveOption::Default
			};
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	
	
	#[test]
	fn template_from_string(){
		assert_eq!(Template::deserialize(json!("grass")).unwrap(), Template::empty("grass"));
	}
	
	#[test]
	fn template_with_kwarg(){
		assert_eq!(
			Template::deserialize(json!({"type": "wall", "kwargs": {"health": 50}})).unwrap(),
			Template::new("wall", hashmap!{"health".to_string() => Parameter::Int(50)})
		);
	}
}
