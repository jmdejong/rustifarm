
use std::collections::HashMap;
use specs::{Builder, EntityBuilder};

use crate::components::{Visible, Blocking, Played};
use crate::hashmap;
use crate::parameter::{Parameter, ParameterType};


#[derive(Clone)]
pub enum ComponentWrapper{
	Visible(Visible),
	Blocking(Blocking),
	Player(Played)
}

impl ComponentWrapper {

	pub fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
		match self.clone() {
			Self::Visible(c) => builder.with(c),
			Self::Blocking(c) => builder.with(c),
			Self::Player(c) => builder.with(c)
		}
	}

	pub fn load_component(comptype: ComponentType, mut parameters: HashMap<&str, Parameter>) -> Option<Self> {
		match comptype {
			ComponentType::Visible => Some(Self::Visible(Visible{
				sprite: parameters.remove("sprite")?.as_str()?.to_string(),
				height: parameters.remove("height")?.as_f64()?
			})),
			ComponentType::Blocking => Some(Self::Blocking(Blocking)),
			ComponentType::Player => Some(Self::Player(Played::new(
				parameters.remove("name")?.as_str()?.to_string()
			)))
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ComponentType {
	Visible,
	Blocking,
	Player
}

impl ComponentType {
	
	pub fn from_str(typename: &str) -> Option<ComponentType>{
		match typename {
			"Visible" => Some(ComponentType::Visible),
			"Blocking" => Some(ComponentType::Blocking),
			"Player" => Some(ComponentType::Player),
			_ => None
		}
	}
	
	pub fn parameters(&self) -> HashMap<&str, ParameterType> {
		match self {
			ComponentType::Visible => hashmap!("sprite" => ParameterType::String, "height" => ParameterType::Float),
			ComponentType::Blocking => HashMap::new(),
			ComponentType::Player => hashmap!("name" => ParameterType::String)
		}
	}
}







