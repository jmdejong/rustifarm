
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
	item::ItemAction,
	PResult,
	perr,
	parameter::Parameter
};

#[derive(Default, Clone)]
pub struct Encyclopedia {
	assemblages: HashMap<EntityType, Assemblage>,
	items: HashMap<ItemId, Item>
}

impl Encyclopedia {
	
	pub fn from_json(val: Value) -> PResult<Encyclopedia> {
		let mut assemblages = 
			val
			.get("assemblages")
			.ok_or(perr!("no assemblages in encyclopedia json"))?
			.as_object()
			.ok_or(perr!("encyclopedia assemblages not a json object"))?
			.into_iter()
			.map(|(k, v)| Ok((EntityType(k.clone()), Assemblage::from_json(v)?)))
			.collect::<PResult<HashMap<EntityType, Assemblage>>>()?;
		let items =
			val
			.get("items")
			.unwrap_or(&json!({}))
			.as_object()
			.ok_or(perr!("encyclopedia items not a json object"))?
			.into_iter()
			.map(|(k, v)| {
				let id = ItemId(k.clone());
				let sprite = 
					if let Some(sprite) = v.get("sprite") {
						sprite.as_str().ok_or(perr!("item sprite not a string: {:?}", v))?
					} else {
						k
					};
				let name =
					if let Some(name) = v.get("name") {
						name.as_str().ok_or(perr!("item name not a string: {:?}", v))?.to_string()
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
							ItemAction::from_json(action).ok_or(perr!("failed to parse ItemAction: {:?}", v))?
						} else {
							ItemAction::None
						}
				};
				Ok((id, item))
			})
			.collect::<PResult<HashMap<ItemId, Item>>>()?;
		for (name, v) in
				val
				.get("templates")
				.unwrap_or(&json!({}))
				.as_object().ok_or(perr!("templates not a json dict: {:?}", val.get("templates")))?
				.iter() {
			let enttype = EntityType(v
				.get(0).ok_or(perr!("index 0 not in subtitution template"))?
				.as_str().ok_or(perr!("subtitution origin name not a string"))?
				.to_string());
			let values = v.get(1).ok_or(perr!("index 0 not in subtitution template"))?;
			let mut assemblage = assemblages.get(&enttype).ok_or(perr!("template name '{:?}' does not point to not an assemblage", enttype))?.clone();
			for arg in assemblage.arguments.iter_mut() {
				if let Some(x) = values.get(&arg.0) {
					let param = Parameter::from_typed_json(arg.1, x)?;
					arg.2 = Some(param);
				}
			}
			assemblages.insert(EntityType(name.to_string()), assemblage);
		}
		
		Ok(Encyclopedia{
			assemblages,
			items
		})
	}
	
	pub fn validate(&self) -> Result<()> {
		for assemblage in self.assemblages.values() {
			assemblage.validate()?;
		}
		Ok(())
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
	
	pub fn merge(mut self, mut other: Encyclopedia) -> Encyclopedia {
		self.assemblages.extend(other.assemblages.drain());
		self.items.extend(other.items.drain());
		self
	}
}

