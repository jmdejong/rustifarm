
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
	items: HashMap<ItemId, Item>,
	assemblage_substitute: HashMap<EntityType, EntityType>,
	item_substitute: HashMap<ItemId, ItemId>,
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
			.ok_or(perr!("no items in encyclopedia json"))?
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
			let mut assemblage = assemblages.get(&enttype).ok_or(perr!("template name '{:?}' points not an assemblage", enttype))?.clone();
			for arg in assemblage.arguments.iter_mut() {
				if let Some(x) = values.get(&arg.0) {
					let param = Parameter::from_typed_json(arg.1, x).ok_or(perr!("subtitution parameter has wrong type"))?;
					arg.2 = Some(param);
				}
			}
			assemblages.insert(EntityType(name.to_string()), assemblage);
		}
		
		let assemblage_substitute = val
			.get("assemblage_substitute")
			.unwrap_or(&json!({}))
			.as_object().ok_or(perr!("assemblage_subtitutions not a json dict"))?
			.iter()
			.chain(
				val
				.get("substitute")
				.unwrap_or(&json!({}))
				.as_object().ok_or(perr!("substitutions not a json dict"))?
				.iter()
			)
			.map(|(from, into)| {
				Ok((
					EntityType(from.to_string()),
					EntityType(into.as_str().ok_or(perr!("substitution value not a string: {:?}", into))?.to_string())
				))
			})
			.collect::<PResult<HashMap<EntityType, EntityType>>>()?;
		
		let item_substitute = val
			.get("assemblage_substitute")
			.unwrap_or(&json!({}))
			.as_object().ok_or(perr!("assemblage_subtitutions not a json dict"))?
			.iter()
			.chain(
				val
				.get("substitute")
				.unwrap_or(&json!({}))
				.as_object().ok_or(perr!("substitutions not a json dict"))?
				.iter()
			)
			.map(|(from, into)| {
				Ok((
					ItemId(from.to_string()),
					ItemId(into.as_str().ok_or(perr!("substitution value not a string: {:?}", into))?.to_string())
				))
			})
			.collect::<PResult<HashMap<ItemId, ItemId>>>()?;
		Ok(Encyclopedia{
			assemblages,
			items,
			assemblage_substitute,
			item_substitute
		})
	}
	
	pub fn validate(&self) -> Result<()> {
		for assemblage in self.assemblages.values() {
			assemblage.validate()?;
		}
		Ok(())
	}
	
	pub fn construct(&self, template: &Template) -> Result<PreEntity> {
		if let Some(new_name) = self.assemblage_substitute.get(&template.name){
			let mut into = template.clone();
			into.name = new_name.clone();
			return self.construct(&into);
		}
		let assemblage = self.assemblages
			.get(&template.name)
			.ok_or(aerr!("unknown assemblage name: '{:?}' in {:?}", template.name, template))?;
		assemblage.instantiate(template)
	}
	
	pub fn get_item(&self, id: &ItemId) -> Option<Item> {
		let actual_id = if let Some(into) = self.item_substitute.get(id) {
				into
			} else {
				id
			};
		self.items.get(actual_id).map(|item| item.clone())
	}
	
	pub fn substitute_item(&self, id: &ItemId) -> ItemId {
		if let Some(into) = self.item_substitute.get(id) {
			into.clone()
		} else {
			id.clone()
		}
	}
}

