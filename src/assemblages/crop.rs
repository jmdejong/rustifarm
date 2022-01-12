
use std::collections::HashMap;

use crate::{
	components::{
		flags::Flag,
		Flags,
		messages::Trigger,
		Build,
		Timer,
		Interactable,
		Loot,
		Serialise
	},
	componentwrapper::{ComponentWrapper, ComponentType},
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	template::Template,
	Result as AnyResult,
	aerr,
	Timestamp,
	hashset,
};
use super::{basic_components, visible_components};

#[derive(Debug, PartialEq, Clone)]
pub struct CropStage;

impl DynamicAssemblage for CropStage {
	
	fn instantiate(&self, template: &Template, arguments: HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let mut components = visible_components(template, &arguments)?;
		let delay = arguments.get("delay")
			.and_then(i64::from_parameter)
			.ok_or(aerr!("no delay found when instantiating {:?}", template))?;
		let target_time = arguments.get("target_time")
			.and_then(Timestamp::from_parameter);
		components.push(ComponentWrapper::Timer(Timer{
			delay,
			spread: 0.5,
			target_time,
			trigger: Trigger::Change
		}));
		components.push(ComponentWrapper::Flags(Flags(hashset!{Flag::Occupied})));
		let next = arguments.get("next")
			.and_then(Template::from_parameter)
			.ok_or(aerr!("no next stage found when instantiating {:?}", template))?;
		components.push(ComponentWrapper::Build(Build{obj: next}));
		if template.should_save() {
			components.push(ComponentWrapper::Serialise(Serialise{
				template: template.clone(),
				extract: vec![("target_time".to_string(), ComponentType::Timer, "target_time".to_string())]
			}));
		}
		Ok(components)
	}
}



#[derive(Debug, PartialEq, Clone)]
pub struct Harvestable;

impl DynamicAssemblage for Harvestable {
	
	fn instantiate(&self, template: &Template, arguments: HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		
		let mut components = basic_components(template, &arguments)?;
		let loot = arguments.get("loot")
			.and_then(<Vec<(Template, f64)>>::from_parameter)
			.ok_or(aerr!("no loot found when instantiating {:?}", template))?;
		components.push(ComponentWrapper::Loot(Loot{loot}));
		components.push(ComponentWrapper::Interactable(Interactable::Trigger(Trigger::Die)));
		components.push(ComponentWrapper::Flags(Flags(hashset!{Flag::Occupied})));
		Ok(components)
	}
}


