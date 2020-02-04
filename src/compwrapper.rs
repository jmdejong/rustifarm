
use std::collections::HashMap;
use specs::{Builder, EntityBuilder};
use serde_json::Value;
use super::components::{Visible, Blocking, Played};


#[derive(Clone)]
pub enum CompWrapper{
	Visible(Visible),
	Blocking(Blocking),
	Player(Played)
}

impl CompWrapper {

	pub fn build<'a>(&self, builder: specs::EntityBuilder<'a>) -> specs::EntityBuilder<'a> {
		match self.clone() {
			Self::Visible(c) => builder.with(c),
			Self::Blocking(c) => builder.with(c),
			Self::Player(c) => builder.with(c)
		}
	}
	
	pub fn parse_component(data: Value) -> Option<CompWrapper> {
		let a = data.as_array()?;
		if a.len() != 2 {
			return None
		}
		let typename = a[0].as_str()?;
		let params: HashMap<&str, &Value> = a[1].as_object()?.into_iter().map(|(key, val)| (key.as_str(), val)).collect();
		Self::load_component(typename, params)
	}

	pub fn load_component(typename: &str, mut parameters: HashMap<&str, &Value>) -> Option<CompWrapper> {
		match typename {
			"Visible" => Some(CompWrapper::Visible(Visible{
				sprite: parameters.remove("sprite")?.as_str()?.to_string(),
				height: parameters.remove("height")?.as_f64()?
			})),
			"Blocking" => Some(CompWrapper::Blocking(Blocking)),
			"Player" => Some(CompWrapper::Player(Played::new(
				parameters.remove("name")?.as_str()?.to_string()
			))),
			_ => None
		}
	}
}
