
use std::collections::HashMap;
use specs::{Component, FlaggedStorage, HashMapStorage};
use crate::{
	ItemId,
	item::{Item, ItemAction},
	components::equipment::{Stat, Equippable},
	Encyclopedia,
	Sprite
};

#[derive(Debug, Clone)]
pub struct InventoryEntry {
	pub itemid: ItemId,
	pub item: Item,
	pub is_equipped: bool
}

#[derive(Debug, Clone, Default)]
pub struct Inventory {
	pub items: Vec<InventoryEntry>,
	pub capacity: usize
}
impl Component for Inventory {
	type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}

impl Inventory {
	
	pub fn add_item(&mut self, itemid: ItemId, enc: &Encyclopedia) {
		self.items.insert(0, InventoryEntry{
			itemid: itemid.clone(),
			item: enc.get_item(&itemid).unwrap(),
			is_equipped: false
		});
	}
	
	fn equipped(&self) -> Vec<Equippable> {
		let mut equippables = Vec::new();
		for entry in self.items.iter() {
			if entry.is_equipped {
				if let ItemAction::Equip(equippable) = &entry.item.action {
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
	
	pub fn equipment_sprites(&self) -> Vec<Sprite> {
		self.equipped().iter().filter_map(|e| e.sprite.clone()).collect()
	}
}
