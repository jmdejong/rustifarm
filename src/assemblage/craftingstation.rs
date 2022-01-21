
use std::collections::HashMap;

use crate::{
	components::{
		Interactable
	},
	componentwrapper::ComponentWrapper,
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	template::Template,
	Result as AnyResult,
	aerr,
	item::ItemId,
	exchange::Exchange
};
use super::basic::{Visible, TemplateSave};

#[derive(Debug, PartialEq, Clone)]
pub struct CraftingStation;

impl DynamicAssemblage for CraftingStation {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		
		let (prefix, trades) = arguments.get("exchanges")
			.and_then(<(String, Vec<(String, Vec<ItemId>, Vec<ItemId>)>)>::from_parameter)
			.ok_or(aerr!("no exchanges found when instantiating {:?}", template))?;
		let exchanges = trades.into_iter().map(|(k, cost, offer)| (k, Exchange{cost, offer})).collect();
		let components = [
			Visible.instantiate(template, arguments)?,
			TemplateSave.instantiate(template, arguments)?,
			vec![
				ComponentWrapper::Interactable(Interactable::Exchange(prefix, exchanges))
			]
		].concat();
		
		Ok(components)
	}
}

