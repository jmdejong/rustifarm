mod basic;
mod randomsprite;
mod letter;
mod configured;
mod item;
mod crop;

use std::collections::HashMap;
use crate::{
	assemblage::Assemblage,
	template::EntityType,
	hashmap
};

use basic::{basic_components, visible_components};
pub use configured::ConfiguredAssemblage;
pub use item::ItemAssemblage;

pub fn default_assemblages() -> HashMap<EntityType, Assemblage> {
	hashmap!{
		EntityType("basic".to_string()) => Assemblage::new(Box::new(basic::BasicAssemblage)),
		EntityType("randomsprite".to_string()) => Assemblage::new(Box::new(randomsprite::RandomSprite)),
		EntityType("letter".to_string()) => Assemblage::new(Box::new(letter::Letter)),
		EntityType("cropstage".to_string()) => Assemblage::new(Box::new(crop::CropStage)),
		EntityType("harvestable".to_string()) => Assemblage::new(Box::new(crop::Harvestable))
	}
}
