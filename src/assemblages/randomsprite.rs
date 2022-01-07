
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
pub struct RandomSprite {
	sprites: Vec<String>,
	name: Option<String>,
	height: f64,
	flags: HashSet<Flag>,
	description: Option<String>
}

impl RandomSprite {

	pub fn new() -> RandomSprite {
		RandomSprite {
			sprites: Vec::new(),
			name: None,
			height: 0.0,
			flags: HashSet::new(),
			description: None
		}
	}
}

impl DynamicAssemblage for RandomSprite {
	
	fn instantiate(&self, template: &Template) -> AnyResult<Vec<ComponentWrapper>> {
		let mut assemblage = self.clone();
		assemblage.apply_arguments(&template.kwargs);
		let name = assemblage.name.clone().ok_or(aerr!("no name found when instantiating {:?}", template))?;
		if assemblage.sprites.is_empty() {
			return Err(aerr!("no sprites found when instantiating {:?}", template));
		}
		let sprite = assemblage.sprites[rand::thread_rng().gen_range(0, assemblage.sprites.len())].clone();
		let mut components = vec![
			ComponentWrapper::Visible(Visible{
				name: name,
				sprite: Sprite(sprite),
				height: assemblage.height
			})
		];
		if !assemblage.flags.is_empty() {
			components.push(ComponentWrapper::Flags(Flags(assemblage.flags.clone())));
		}
		if let Some(actual_description) = assemblage.description {
			components.push(ComponentWrapper::Description(Description{description: actual_description.clone()}));
		}
		Ok(components)
	}
	
	fn apply_arguments(&mut self, arguments: &HashMap<String, Parameter>) {
		let assemblage = self;
		let mut sprites = arguments.get("sprites")
			.cloned()
			.and_then(<Vec<String>>::from_parameter)
			.unwrap_or_else(Vec::new);
		assemblage.sprites.append(&mut sprites);
		assemblage.name = arguments.get("name")
			.cloned()
			.and_then(String::from_parameter)
			.or(assemblage.name.clone());
		assemblage.height = arguments.get("height")
			.cloned()
			.and_then(f64::from_parameter)
			.unwrap_or(assemblage.height);
		assemblage.description = arguments.get("description")
			.cloned()
			.and_then(String::from_parameter)
			.or(assemblage.description.clone());
		let flags: HashSet<Flag> = arguments.get("flags")
			.cloned()
			.and_then(<HashSet<Flag>>::from_parameter)
			.unwrap_or_else(HashSet::new);
		assemblage.flags = assemblage.flags.union(&flags).cloned().collect();
	}
}
