
use std::collections::{HashMap};
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use dyn_clonable::*;
use crate::{
	parameter::{Parameter},
	componentwrapper::{ComponentWrapper},
	Template,
	Result as AnyResult,
	sprite::Sprite,
	assemblages::{ConfiguredAssemblage, ItemAssemblage},
	item::ItemId
};


#[derive(Debug, Clone)]
pub struct Assemblage {
	assemblage: Box<dyn DynamicAssemblage>,
	arguments: HashMap<String, Parameter>
}

impl Assemblage {

	pub fn new(assemblage: Box<dyn DynamicAssemblage>) -> Self {
		Self{assemblage, arguments: HashMap::new()}
	}
	
	pub fn new_item(id: String, sprite: Sprite, name: String) -> Assemblage {
		Assemblage::new(Box::new(ItemAssemblage::new(ItemId(id), name, sprite)))
	}
	
	pub fn validate(&self) -> AnyResult<()> {
		self.assemblage.validate()
	}
	
	
	pub fn instantiate(&self, template: &Template) -> AnyResult<Vec<ComponentWrapper>>{
		let mut arguments = self.arguments.clone();
		for (key, value) in template.kwargs.clone() {
			arguments.insert(key, value);
		}
		self.assemblage.instantiate(template, arguments)
	}
	
	pub fn apply_arguments(&self, arguments: HashMap<String, Parameter>) -> Self {
		let mut assemblage = self.clone();
		for (key, value) in arguments {
			assemblage.arguments.insert(key, value);
		}
		assemblage
	}
}


impl<'de> Deserialize<'de> for Assemblage {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		Ok(Assemblage::new(Box::new(ConfiguredAssemblage::deserialize(deserializer)?)))
	}
}

#[clonable]
pub trait DynamicAssemblage: Debug + Clone + Send + Sync {
	fn validate(&self) -> AnyResult<()> {
		Ok(())
	}
	
	fn instantiate(&self, template: &Template, arguments: HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>>;
}

