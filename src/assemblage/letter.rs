
use std::collections::HashMap;

use crate::{
	components::Visible,
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
pub struct Letter;

impl DynamicAssemblage for Letter {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let character = &arguments.get("char")
			.and_then(String::from_parameter)
			.ok_or(aerr!("no character found when instantiating letter {:?}", template))?;
		let description = arguments.get("description")
			.and_then(String::from_parameter);
		Ok(vec![
			ComponentWrapper::Visible(Visible{
				name: format!("letter '{}'", character),
				sprite: Sprite(format!("emptyletter-{}", character)),
				height: 1.0,
				description
			})
		])
	}
}
