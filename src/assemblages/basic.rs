
use std::collections::{HashSet, HashMap};

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
pub struct BasicAssemblage {
	sprite: Option<String>,
	name: Option<String>,
	height: f64,
	flags: HashSet<Flag>,
	description: Option<String>
}

impl BasicAssemblage {

	pub fn new() -> BasicAssemblage {
		BasicAssemblage {
			sprite: None,
			name: None,
			height: 0.0,
			flags: HashSet::new(),
			description: None
		}
	}
}

impl DynamicAssemblage for BasicAssemblage {
	
	fn instantiate(&self, template: &Template) -> AnyResult<Vec<ComponentWrapper>> {
		let mut assemblage = self.clone();
		assemblage.apply_arguments(&template.kwargs);
		let BasicAssemblage{sprite, name, height, flags, description} = assemblage;
		let actual_name = name.or(sprite.clone()).ok_or(aerr!("no sprite or name found when instantiating {:?}", template))?;
		let mut components = vec![
			ComponentWrapper::Visible(Visible{
				name: actual_name.clone(),
				sprite: Sprite(sprite.unwrap_or(actual_name)),
				height
			})
		];
		if !flags.is_empty() {
			components.push(ComponentWrapper::Flags(Flags(flags)));
		}
		if let Some(actual_description) = description {
			components.push(ComponentWrapper::Description(Description{description: actual_description}));
		}
		Ok(components)
	}
	
	fn apply_arguments(&mut self, arguments: &HashMap<String, Parameter>) {
		let assemblage = self;
		assemblage.sprite = arguments.get("sprite")
			.cloned()
			.and_then(String::from_parameter)
			.or(assemblage.sprite.clone());
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
