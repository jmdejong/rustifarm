
use std::collections::HashMap;
use serde::{Deserialize};
use serde_json::{json, from_value};

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
	template::Template,
	Result as AnyResult,
	Timestamp,
	hashset,
};
use super::basic::{Visible, TemplateSave};


#[derive(Deserialize)]
pub struct CropStageArgs {
	pub delay: i64,
	pub target_time: Option<Timestamp>,
	pub next: Template
}

#[derive(Debug, PartialEq, Clone)]
pub struct CropStage;

impl DynamicAssemblage for CropStage {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let args: CropStageArgs = from_value(json!(arguments))?;
		let mut components = Visible.instantiate(template, arguments)?;
		components.push(ComponentWrapper::Timer(Timer{
			delay: args.delay,
			spread: 0.5,
			target_time: args.target_time,
			trigger: Trigger::Change
		}));
		components.push(ComponentWrapper::Flags(Flags(hashset!{Flag::Occupied})));
		components.push(ComponentWrapper::Build(Build{obj: args.next}));
		if template.should_save() {
			components.push(ComponentWrapper::Serialise(Serialise{
				template: template.clone(),
				extract: vec![("target_time".to_string(), ComponentType::Timer, "target_time".to_string())]
			}));
		}
		Ok(components)
	}
}


#[derive(Deserialize)]
pub struct LootArg {
	loot: Vec<(Template, f64)>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Harvestable;

impl DynamicAssemblage for Harvestable {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let arg: LootArg = from_value(json!(arguments))?;
		Ok([
			Visible.instantiate(template, arguments)?,
			TemplateSave.instantiate(template, arguments)?,
			vec![
				ComponentWrapper::Loot(Loot{loot: arg.loot}),
				ComponentWrapper::Interactable(Interactable::Trigger(Trigger::Die)),
				ComponentWrapper::Flags(Flags(hashset!{Flag::Occupied}))
			]
		].concat())
	}
}


