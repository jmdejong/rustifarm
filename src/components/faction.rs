
use strum_macros::{EnumString, Display};
use specs::{
	Component,
	HashMapStorage,
	ReadStorage,
	Entity,
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
#[storage(HashMapStorage)]
pub enum Faction {
	Neutral,
	Good,
	Evil,
	None
}

use Faction::{Neutral, Good, Evil, None};

impl Faction {
	
	pub fn is_enemy(&self, other: Faction) -> bool {
		match self {
			Neutral => false,
			Good => other == Evil || other == None,
			Evil => other == Good || other == None,
			None => other != Neutral
		}
	}
	
	pub fn is_enemy_entity(factions: &ReadStorage<Self>, a: Entity, b: Entity) -> bool{
		factions.get(a).unwrap_or(&None).is_enemy(*factions.get(b).unwrap_or(&None))
	}
}
