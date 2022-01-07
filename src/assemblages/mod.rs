pub mod basic;
pub mod randomsprite;

use std::collections::HashMap;
use crate::{
	assemblage::Assemblage,
	template::EntityType,
	hashmap
};


pub fn default_assemblages() -> HashMap<EntityType, Assemblage> {
	hashmap!{
		EntityType("basic".to_string()) => Assemblage::Dynamic(Box::new(basic::BasicAssemblage::new())),
		EntityType("randomsprite".to_string()) => Assemblage::Dynamic(Box::new(randomsprite::RandomSprite::new()))
	}
}
