
use std::collections::{HashMap};

use crate::{
	components::{Visible, Item, Serialise},
	item::ItemId,
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
pub struct ItemAssemblage {
	sprite: Option<Sprite>,
	name: Option<String>,
	id: Option<ItemId>
}

impl ItemAssemblage {
	pub fn new(id: ItemId, name: String, sprite: Sprite) -> Self {
		Self {
			id: Some(id),
			name: Some(name),
			sprite: Some(sprite)
		}
	}
}

impl DynamicAssemblage for ItemAssemblage {
	fn instantiate(&self, template: &Template, arguments: HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		let sprite = arguments.get("sprite")
			.and_then(Sprite::from_parameter)
			.or(self.sprite.clone())
			.ok_or(aerr!("no sprite found when instantiating {:?}", template))?;
		let name = arguments.get("name")
			.and_then(String::from_parameter)
			.or(self.name.clone())
			.ok_or(aerr!("no name found when instantiating {:?}", template))?;
		let id = arguments.get("itemid")
			.and_then(ItemId::from_parameter)
			.or(self.id.clone())
			.ok_or(aerr!("no item id found when instantiating {:?}", template))?;
		Ok(vec![
			ComponentWrapper::Visible(Visible{
				name,
				sprite,
				height: 0.3_f64
			}),
			ComponentWrapper::Item(Item(id)),
			ComponentWrapper::Serialise(Serialise{template: template.clone(), extract: Vec::new() })
		])
	}
}
