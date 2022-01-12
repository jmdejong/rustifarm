
use std::collections::HashMap;
use serde::{de, Deserialize, Serialize, Deserializer};
use crate::{
	assemblage::{Assemblage},
	assemblages,
	componentwrapper::PreEntity,
	Template,
	template::EntityType,
	Result as AnyResult,
	aerr,
	ItemId,
	item::Item,
	item::ItemAction,
	parameter::Parameter,
	Sprite
};

#[derive(Default, Clone)]
pub struct Encyclopedia {
	assemblages: HashMap<EntityType, Assemblage>,
	items: HashMap<ItemId, Item>
}

impl Encyclopedia {
	
	pub fn validate(&self) -> AnyResult<()> {
		for (name, assemblage) in self.assemblages.iter() {
			assemblage.validate().map_err(|e| aerr!("invalid assemblage {}: {}", name.0, e))?;
		}
		Ok(())
	}
	
	pub fn construct(&self, template: &Template) -> AnyResult<PreEntity> {
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


impl<'de> Deserialize<'de> for Encyclopedia {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let EncyclopediaSave{mut assemblages, items, templates} = EncyclopediaSave::deserialize(deserializer)?;
		for (id, assemblage) in assemblages::default_assemblages().into_iter() {
			assemblages.insert(id, assemblage);
		}
		let mut itemdefs = HashMap::new();
		for (id, item) in items.into_iter(){
			let sprite = item.sprite.unwrap_or(Sprite(id.clone()));
			let name = item.name.unwrap_or(id.clone());
			let ent = item.entity.unwrap_or_else(||{
				let enttyp = EntityType(id.clone());
				assemblages.insert(enttyp.clone(), Assemblage::new_item(id.clone(), sprite, name.clone()));
				Template::from_entity_type(enttyp)
			});
			itemdefs.insert(ItemId(id), Item{
				ent,
				name,
				action: item.action.unwrap_or(ItemAction::None)
			});
		}
		for (templatename, (baseent, args)) in templates {
			let assemblage = assemblages.get(&baseent)
				.ok_or(de::Error::custom(format!("template name '{:?}' does not point to an assemblage", baseent)))?
				.apply_arguments(args);
			assemblages.insert(templatename, assemblage);
		}
		
		Ok(Encyclopedia{
			assemblages,
			items: itemdefs
		})
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ItemSave {
	sprite: Option<Sprite>,
	name: Option<String>,
	entity: Option<Template>,
	action: Option<ItemAction>
}
#[derive(Debug, Clone, Deserialize)]
struct EncyclopediaSave {
	#[serde(default)]
	assemblages: HashMap<EntityType, Assemblage>,
	#[serde(default)]
	items: HashMap<String, ItemSave>,
	#[serde(default)]
	templates: HashMap<EntityType, (EntityType, HashMap<String, Parameter>)>
}
