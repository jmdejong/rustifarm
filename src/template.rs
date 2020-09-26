

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::{
	parameter::Parameter,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct EntityType(pub String);

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
		save: Option<bool>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		clan: Option<String>
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from="TemplateSave", into="TemplateSave")]
pub struct Template {
	pub name: EntityType,
	pub args: Vec<Parameter>,
	pub kwargs: HashMap<String, Parameter>,
	pub save: Option<bool>,
	pub clan: Option<String>
}


impl From<TemplateSave> for Template {
	fn from(ts: TemplateSave) -> Self {
		match ts {
			TemplateSave::Name(name) => Self{name, args: Vec::new(), kwargs: HashMap::new(), save: None, clan: None},
			TemplateSave::Full{name, args, kwargs, save, clan} => Self{name, args, kwargs, save, clan}
		}
	}
}
impl Into<TemplateSave> for Template {
	fn into(self) -> TemplateSave {
		if self.args.is_empty() && self.kwargs.is_empty() && self.save == None && self.clan == None {
			return TemplateSave::Name(self.name);
		}
		TemplateSave::Full {
			name: self.name,
			args: self.args,
			kwargs: self.kwargs,
			save: self.save,
			clan: self.clan
		}
	}
}

impl Template {
	
	pub fn new(name: &str, kwargs: HashMap<String, Parameter>) -> Self {
		Self {
			name: EntityType(name.to_string()),
			args: Vec::new(),
			kwargs,
			save: None,
			clan: None
		}
	}
	
	pub fn empty(name: &str) -> Self {
		Self::new(name, HashMap::new())
	}
	
	pub fn should_save(&self) -> bool {
		match self.save {
			None | Some(true) => true,
			Some(false) => false
		}
	}
	
	pub fn from_entity_type(typ: EntityType) -> Self {
		Self {
			name: typ,
			args: Vec::new(),
			kwargs: HashMap::new(),
			save: None,
			clan: None
		}
	}
	
	pub fn unsaved(mut self) -> Self {
		if self.save == None {
			self.save = Some(false)
		}
		self
	}
	
	pub fn merge(mut self, other: Template) -> Self {
		if self.save == None {
			self.save = other.save;
		}
		if self.clan == None {
			self.clan = other.clan;
		}
		for (key, value) in other.kwargs {
			self.kwargs.entry(key).or_insert(value);
		}
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use serde_json::json;
	
	
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
