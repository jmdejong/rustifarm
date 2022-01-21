
use std::collections::HashMap;

use crate::{
	components::{
		messages::Trigger,
		Timer,
	},
	componentwrapper::ComponentWrapper,
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	template::Template,
	Result as AnyResult,
	aerr,
};
use super::basic::Visible;

#[derive(Debug, PartialEq, Clone)]
pub struct Particle;

impl DynamicAssemblage for Particle {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let mut components = Visible.instantiate(template, arguments)?;
		let duration = arguments.get("duration")
			.and_then(i64::from_parameter)
			.ok_or(aerr!("no delay found when instantiating {:?}", template))?;
		components.push(ComponentWrapper::Timer(Timer{
			delay: duration,
			spread: 0.0,
			target_time: None,
			trigger: Trigger::Remove
		}));
		Ok(components)
	}
}


