

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::{
	parameter::Parameter,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct EntityType(pub String);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SaveOption {
	Default,
	False,
	Always
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
