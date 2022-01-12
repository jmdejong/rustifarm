
use std::collections::{HashSet, HashMap};

use crate::{
	components::{flags::Flag, Flags, Visible, Description, Serialise},
	componentwrapper::ComponentWrapper,
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	sprite::Sprite,
	template::Template,
	Result as AnyResult,
	aerr
};
pub fn visible_components(template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
	let sprite = arguments.get("sprite")
		.and_then(Sprite::from_parameter)
		.ok_or(aerr!("no sprite found when instantiating {:?}", template))?;
	let name = arguments.get("name")
		.and_then(String::from_parameter)
		.unwrap_or_else(|| sprite.0.clone());
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
	if let Some(description) = arguments.get("description").and_then(String::from_parameter) {
		components.push(ComponentWrapper::Description(Description{description}));
	}
	Ok(components)
}

pub fn basic_components(template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
	let mut components = visible_components(template, arguments)?;
	if template.should_save() {
		components.push(ComponentWrapper::Serialise(Serialise{template: template.clone(), extract: Vec::new()}));
	}
	Ok(components)
}


#[derive(Debug, PartialEq, Clone)]
pub struct BasicAssemblage;

impl DynamicAssemblage for BasicAssemblage {
	
	fn instantiate(&self, template: &Template, arguments: HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let mut components = basic_components(template, &arguments)?;
		if let Some(flags) = arguments.get("flags").and_then(<HashSet<Flag>>::from_parameter) {
			components.push(ComponentWrapper::Flags(Flags(flags)));
		}
		Ok(components)
	}
}
