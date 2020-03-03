
use specs::{
	Component,
	HashMapStorage,
	ReadStorage,
	Entity,
};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub enum Faction {
	Neutral,
	Good,
	Evil,
	None
}

use Faction::{Neutral, Good, Evil, None};

impl Faction {
	
	pub fn from_str(name: &str) -> Option<Faction> {
		match name.to_lowercase().as_str() {
			"neutral" => Some(Neutral),
			"good" => Some(Good),
			"evil" => Some(Evil),
			"none" => Some(None),
			"" => Some(None),
			_ => Option::None
		}
	}
	
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
