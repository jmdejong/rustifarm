
use std::collections::HashMap;
use serde_json::{Value, json};
use crate::{
	assemblage::Assemblage,
	componentwrapper::PreEntity,
	Template,
	template::EntityType,
	Result,
	aerr,
	ItemId,
	item::Item,
	item::ItemAction
};

#[derive(Default, Clone)]
pub struct Encyclopedia {
	assemblages: HashMap<EntityType, Assemblage>,
	items: HashMap<ItemId, Item>
}

impl Encyclopedia {
	
	pub fn from_json(val: Value) -> Result<Encyclopedia> {
		let mut assemblages = 
			val
			.get("assemblages")
			.ok_or(aerr!("no assemblages in encyclopedia json"))?
			.as_object()
			.ok_or(aerr!("encyclopedia assemblages not a json object"))?
			.into_iter()
			.map(|(k, v)| Ok((EntityType(k.clone()), Assemblage::from_json(v)?)))
			.collect::<Result<HashMap<EntityType, Assemblage>>>()?;
		let items =
			val
			.get("items")
			.ok_or(aerr!("no items in encyclopedia json"))?
			.as_object()
			.ok_or(aerr!("encyclopedia items not a json object"))?
			.into_iter()
			.map(|(k, v)| {
				let id = ItemId(k.clone());
				let sprite = 
					if let Some(sprite) = v.get("sprite") {
						sprite.as_str().ok_or(aerr!("item sprite not a string: {:?}", v))?
					} else {
						k
					};
				let name =
					if let Some(name) = v.get("name") {
						name.as_str().ok_or(aerr!("item name not a string: {:?}", v))?.to_string()
					} else {
						k.to_string()
					};
				let item = Item {
					name: name.clone(),
					ent:
						if let Some(ent) = v.get("entity") {
							Template::from_json(ent)?
						} else {
							let enttyp = EntityType(k.clone());
							assemblages.insert(enttyp.clone(), Assemblage::from_json(&json!({
								"height": 0.3,
								"sprite": sprite,
								"name": name,
								"item": k
							}))?);
							Template::from_entity_type(enttyp)
						},
					action: 
						if let Some(action) = v.get("action") {
							ItemAction::from_json(action).ok_or(aerr!("failed to parse ItemAction: {:?}", v))?
						} else {
							ItemAction::None
						}
				};
				Ok((id, item))
			})
			.collect::<Result<HashMap<ItemId, Item>>>()?;
		
		Ok(Encyclopedia{
			assemblages,
			items
		})
	}
	
	pub fn construct(&self, template: &Template) -> Result<PreEntity> {
		let assemblage = self.assemblages
			.get(&template.name)
			.ok_or(aerr!("unknown assemblage name: '{:?}' in {:?}", template.name, template))?;
		assemblage.instantiate(template)
	}
	
	pub fn get_item(&self, id: &ItemId) -> Option<Item> {
		self.items.get(id).map(|item| item.clone())
	}
}

