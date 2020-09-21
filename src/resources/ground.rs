
use std::collections::{HashMap, HashSet};

use specs::{
	ReadStorage,
	Component,
	Entity
};

use crate::{
	components::{Visible, Flags, Flag},
	Pos,
	controls::Direction
};

#[derive(Default)]
pub struct Ground {
	pub cells: HashMap<Pos, HashSet<Entity>>,
	pub changes: HashSet<Pos>
}

impl Ground {
	
	pub fn insert(&mut self, pos: Pos, ent: Entity){
		self.cells.entry(pos).or_insert_with(HashSet::new).insert(ent);
		self.changes.insert(pos);
	}
	
	pub fn remove(&mut self, pos: &Pos, ent: Entity) -> bool{
		if let Some(cell) = self.cells.get_mut(pos) {
			self.changes.insert(*pos);
			cell.remove(&ent)
		} else {
			false
		}
	}
	
	pub fn components_on<'a, C: Component>(&self, pos: Pos, component_type: &'a ReadStorage<C>) -> Vec<&'a C> {
		self.cells
			.get(&pos)
			.unwrap_or(&HashSet::new())
			.iter()
			.filter_map(|e| component_type.get(*e))
			.collect()
	}
	
	pub fn components_near<'a, C: Component>(&self, pos: Pos, directions: &[Direction], component_type: &'a ReadStorage<C>) -> Vec<(Entity, &'a C)> {
		let mut nearby_components: Vec<(Entity, &'a C)> = Vec::new();
		for direction in directions {
			let pos = pos + direction.to_position();
			for ent in self.cells.get(&pos).unwrap_or(&HashSet::new()) {
				if let Some(comp) = component_type.get(*ent) {
					nearby_components.push((*ent, comp));
				}
			}
		}
		nearby_components
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
		self.components_on::<Flags>(pos, flags).into_iter().fold(HashSet::new(), |a, b| &a | &b.0)
	}
}
