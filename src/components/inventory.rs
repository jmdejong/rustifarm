
use std::collections::HashMap;
use specs::{Component, FlaggedStorage, HashMapStorage};
use super::{
	item::{Item, ItemAction},
	equipment::{Stat, Equippable},
};

#[derive(Debug, Clone, Default)]
pub struct Inventory {
	pub items: Vec<(Item, bool)>,
	pub capacity: usize
}
impl Component for Inventory {
	type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}

impl Inventory {
	
	fn equipped(&self) -> Vec<Equippable> {
		let mut equippables = Vec::new();
		for (item, is_equipped) in self.items.iter() {
			if *is_equipped {
				if let ItemAction::Equip(equippable) = &item.action {
					equippables.push(equippable.clone());
				} else {
					panic!("unequippable item equipped!");
				}
			}
		}
		equippables
	}
	
	pub fn equipment_bonuses(&self) -> HashMap<Stat, i64> {
		let mut bonuses: HashMap<Stat, i64> = HashMap::new();
		for equippable in self.equipped() {
			for (stat, s) in equippable.stats.iter(){
				let current: i64 = *bonuses.entry(*stat).or_insert(0);
				bonuses.insert(*stat, current + s);
			}
		}
		bonuses
	}
}
