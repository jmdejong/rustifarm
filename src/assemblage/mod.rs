mod basic;
mod randomsprite;
mod letter;
mod configured;
mod item;
mod crop;
mod portal;
mod particle;
mod craftingstation;

use std::collections::HashMap;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use enum_dispatch::enum_dispatch;
use crate::{
	template::EntityType,
	hashmap,
	parameter::{Parameter},
	componentwrapper::ComponentWrapper,
	components::Clan,
	Template,
	Result as AnyResult,
	sprite::Sprite,
	item::ItemId
};


pub use configured::ConfiguredAssemblage;
pub use item::ItemAssemblage;
pub use basic::BasicAssemblage;
pub use randomsprite::RandomSprite;
pub use letter::Letter;
pub use crop::{CropStage, Harvestable};
pub use portal::{Portal, HomePortal};
pub use particle::Particle;
pub use craftingstation::CraftingStation;




#[enum_dispatch]
pub trait DynamicAssemblage {
	fn validate(&self) -> AnyResult<()> {
		Ok(())
	}
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>>;
}

#[enum_dispatch(DynamicAssemblage)]
#[derive(Debug, Clone, PartialEq)]
pub enum AssemblageType {
	BasicAssemblage,
	RandomSprite,
	Letter,
	CropStage,
	Harvestable,
	ConfiguredAssemblage,
	ItemAssemblage,
	Portal,
	HomePortal,
	Particle,
	CraftingStation
}



#[derive(Debug, Clone, PartialEq)]
pub struct Assemblage {
	assemblage: AssemblageType,
	arguments: HashMap<String, Parameter>
}

impl Assemblage {

	pub fn new(assemblage: AssemblageType) -> Self {
		Self{assemblage, arguments: HashMap::new()}
	}
	
	pub fn new_item(id: String, sprite: Sprite, name: String) -> Assemblage {
		Assemblage::new(ItemAssemblage::new(ItemId(id), name, sprite).into())
	}
	
	pub fn validate(&self) -> AnyResult<()> {
		self.assemblage.validate()
	}
	
	
	pub fn instantiate(&self, template: &Template) -> AnyResult<Vec<ComponentWrapper>>{
		let mut arguments = self.arguments.clone();
		for (key, value) in template.kwargs.clone() {
			arguments.insert(key, value);
		}
		let mut components = self.assemblage.instantiate(template, &arguments)?;
		if let Some(clan) = &template.clan {
			components.push(ComponentWrapper::Clan(Clan{name: clan.clone()}));
		}
		Ok(components)
	}
	
	pub fn apply_arguments(&self, arguments: HashMap<String, Parameter>) -> Self {
		let mut assemblage = self.clone();
		for (key, value) in arguments {
			assemblage.arguments.insert(key, value);
		}
		assemblage
	}
}


impl<'de> Deserialize<'de> for Assemblage {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		Ok(Assemblage::new(ConfiguredAssemblage::deserialize(deserializer)?.into()))
	}
}

pub fn default_assemblages() -> HashMap<EntityType, Assemblage> {
	hashmap!{
		EntityType("basic".to_string()) => Assemblage::new(BasicAssemblage.into()),
		EntityType("randomsprite".to_string()) => Assemblage::new(RandomSprite.into()),
		EntityType("letter".to_string()) => Assemblage::new(Letter.into()),
		EntityType("cropstage".to_string()) => Assemblage::new(CropStage.into()),
		EntityType("harvestable".to_string()) => Assemblage::new(Harvestable.into()),
		EntityType("portal".to_string()) => Assemblage::new(Portal.into()),
		EntityType("_homeportal".to_string()) => Assemblage::new(HomePortal.into()),
		EntityType("particle".to_string()) => Assemblage::new(Particle.into()),
		EntityType("craftingstation".to_string()) => Assemblage::new(CraftingStation.into())
	}
}
