
use std::collections::{HashMap, HashSet};

use specs::{
	ReadStorage,
	Component,
	Entity
};

use crate::{
	components::{Visible, Removed, Flags, Flag},
	Pos
};

#[derive(Default)]
pub struct Ground {
	pub cells: HashMap<Pos, HashSet<Entity>>
}

impl Ground {
	pub fn components_on<'a, C: Component>(&self, pos: Pos, component_type: &'a ReadStorage<C>, removals: &'a ReadStorage<Removed>) -> Vec<&'a C> {
		self.cells
			.get(&pos)
			.unwrap_or(&HashSet::new())
			.iter()
			.filter(|e| !removals.contains(**e))
			.filter_map(|e| component_type.get(*e))
			.collect()
	}
	
	pub fn all_components_on<'a, C: Component>(&self, pos: Pos, component_type: &'a ReadStorage<C>) -> Vec<&'a C> {
		self.cells
			.get(&pos)
			.unwrap_or(&HashSet::new())
			.iter()
			.filter_map(|e| component_type.get(*e))
			.collect()
	}
	
	pub fn by_height(&self, pos: &Pos, visibles: &ReadStorage<Visible>, ignore: &Entity) -> Vec<Entity> {
		let mut entities: Vec<Entity> = self.cells
			.get(&pos).unwrap_or(&HashSet::new())
			.iter()
			.cloned()
			.filter(|e| e != ignore && visibles.contains(*e))
			.collect();
		entities.sort_by(|a, b|
			visibles.get(*b).unwrap().height.partial_cmp(&visibles.get(*a).unwrap().height).unwrap()
		);
		entities
	}
	
	pub fn flags_on<'a>(&self, pos: Pos, flags: &'a ReadStorage<Flags>) -> HashSet<Flag> {
		self.all_components_on::<Flags>(pos, flags).into_iter().fold(HashSet::new(), |a, b| &a | &b.0)
	}
}
