
use std::collections::{HashSet, HashMap};

use crate::{
	components::{flags::Flag, Flags, Visible as VisibleComponent, Serialise},
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
pub struct Visible;

impl DynamicAssemblage for Visible {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let sprite = arguments.get("sprite")
			.and_then(Sprite::from_parameter)
			.ok_or(aerr!("no sprite found when instantiating {:?}", template))?;
		let name = arguments.get("name")
			.and_then(String::from_parameter)
			.unwrap_or_else(|| sprite.0.clone());
		let height = arguments.get("height")
			.and_then(f64::from_parameter)
			.ok_or(aerr!("no height found when instantiating {:?}", template))?;
		let description = arguments.get("description")
			.and_then(String::from_parameter);
		Ok(vec![
			ComponentWrapper::Visible(VisibleComponent{
				name,
				sprite,
				height,
				description
			})
		])
	}
}



#[derive(Debug, PartialEq, Clone)]
pub struct MaybeVisible;

impl DynamicAssemblage for MaybeVisible {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		 Ok(Visible.instantiate(template, arguments).unwrap_or(Vec::new()))
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct TemplateSave;


impl DynamicAssemblage for TemplateSave {
	
	fn instantiate(&self, template: &Template, _arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		Ok(if template.should_save() {
			vec![ComponentWrapper::Serialise(Serialise{template: template.clone(), extract: Vec::new()})]
		} else {
			Vec::new()
		})
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct BasicAssemblage;

impl DynamicAssemblage for BasicAssemblage {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let mut components = [
			Visible.instantiate(template, arguments)?,
			TemplateSave.instantiate(template, arguments)?
			].concat();
		if let Some(flags) = arguments.get("flags").and_then(<HashSet<Flag>>::from_parameter) {
			components.push(ComponentWrapper::Flags(Flags(flags)));
		}
		Ok(components)
	}
}
