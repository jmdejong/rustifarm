
use std::collections::{HashSet, HashMap};
use rand::Rng;

use crate::{
	components::{flags::Flag, Flags, Visible, Description},
	componentwrapper::ComponentWrapper,
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	sprite::Sprite,
	template::Template,
	Result as AnyResult,
	aerr
};

#[derive(Debug, PartialEq, Clone)]
pub struct RandomSprite;

impl DynamicAssemblage for RandomSprite {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let name = arguments.get("name")
			.and_then(String::from_parameter)
			.ok_or(aerr!("no name found when instantiating {:?}", template))?;
		let sprites = arguments.get("sprites")
			.and_then(<Vec<Sprite>>::from_parameter)
			.filter(|sprites| !sprites.is_empty())
			.ok_or(aerr!("no sprites found when instantiating {:?}", template))?;
		let sprite = sprites[rand::thread_rng().gen_range(0, sprites.len())].clone();
		let height = arguments.get("height")
			.and_then(f64::from_parameter)
			.ok_or(aerr!("no height found when instantiating {:?}", template))?;
		let mut components = vec![
			ComponentWrapper::Visible(Visible{
				name,
				sprite,
				height
			})
		];
		if let Some(flags) = arguments.get("flags").and_then(<HashSet<Flag>>::from_parameter) {
			components.push(ComponentWrapper::Flags(Flags(flags)));
		}
		if let Some(description) = arguments.get("description").and_then(String::from_parameter) {
			components.push(ComponentWrapper::Description(Description{description}));
		}
		Ok(components)
	}
}
