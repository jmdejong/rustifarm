
use std::collections::HashSet;

use specs::{
	Read,
	WriteStorage,
	ReadStorage,
	Entities,
	Entity,
	System,
	Join
};

use crate::{
	components::{Dedup, Removed, New, Position},
	resources::Ground
};


pub struct Deduplicate;
impl <'a> System<'a> for Deduplicate {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Dedup>,
		WriteStorage<'a, Removed>,
		ReadStorage<'a, New>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>
	);
	fn run(&mut self, (entities, mut dedups, mut removeds, news, positions, ground): Self::SystemData) {
		for (entity, dedup, position, _) in (&entities, &dedups, &positions, &news).join() {
			let others: Vec<(Entity, &Dedup)> = ground.cells
				.get(&position.pos)
				.unwrap_or(&HashSet::new())
				.iter()
				.filter_map(|e| Some((*e, dedups.get(*e)?)))
				.collect();
			for (e, d) in others {
				if dedup.id == d.id {
					if dedup.priority > d.priority {
						removeds.insert(e, Removed).unwrap();
					} else if dedup.priority < d.priority {
						removeds.insert(entity, Removed).unwrap();
					} else if entity > e {
						removeds.insert(e, Removed).unwrap();
					} else if entity < e {
						removeds.insert(entity, Removed).unwrap();
					}
				}
			}
		}
		for (dedup, _) in (&mut dedups, &news).join() {
			dedup.priority += 1;
		}
	}
}

