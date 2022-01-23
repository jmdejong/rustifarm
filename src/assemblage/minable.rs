
use std::collections::HashMap;

use crate::{
	components,
	components::{
		messages::Trigger,
		Interactable,
		Loot,
		equipment::Stat
	},
	componentwrapper::ComponentWrapper,
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	template::Template,
	Result as AnyResult,
	aerr
};
use super::basic::{BasicAssemblage};




#[derive(Debug, PartialEq, Clone)]
pub struct Minable;

impl DynamicAssemblage for Minable {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		
		let loot = arguments.get("loot")
			.and_then(<Vec<(Template, f64)>>::from_parameter)
			.ok_or(aerr!("no loot found when instantiating {:?}", template))?;
		let stat = arguments.get("stat")
			.and_then(<Stat>::from_parameter)
			.ok_or(aerr!("no stat found when instantiating {:?}", template))?;
		let trigger = arguments.get("trigger")
			.and_then(<Trigger>::from_parameter)
			.ok_or(aerr!("no trigger found when instantiating {:?}", template))?;
		let effort = arguments.get("effort")
			.and_then(<i64>::from_parameter)
			.ok_or(aerr!("no effort found when instantiating {:?}", template))?;
		Ok([
			BasicAssemblage.instantiate(template, arguments)?,
			vec![
				ComponentWrapper::Loot(Loot{loot}),
				ComponentWrapper::Interactable(Interactable::Mine(stat)),
				ComponentWrapper::Minable(components::Minable{
					trigger,
					total: effort,
					progress: 0
				})
			]
		].concat())
	}
}


