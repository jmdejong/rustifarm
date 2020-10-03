
use std::collections::HashSet;
use specs::{
	WriteStorage,
	ReadStorage,
	Entities,
	System,
	Join,
	Read
};

use crate::{
	components::{Removed, Requirements, New, Position, Flag, Flags},
	resources::{Ground, RoomFlags}
};


pub struct SpawnCheck;
impl <'a> System<'a> for SpawnCheck {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Requirements>,
		ReadStorage<'a, New>,
		WriteStorage<'a, Removed>,
		Read<'a, Ground>,
		Read<'a, RoomFlags>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Flags>
	);
	fn run(&mut self, (entities, requirements, new, mut removeds, ground, roomflags, positions, flags): Self::SystemData) {
		for (entity, requirements, _new, position) in (&entities, &requirements, &new, &positions).join() {
			let ground_flags: HashSet<Flag> = ground.flags_on(position.pos, &flags).union(&roomflags.0).cloned().collect();
			if !(requirements.required_flags.is_subset(&ground_flags) && requirements.blocking_flags.is_disjoint(&ground_flags)){
				removeds.insert(entity, Removed).unwrap();
			}
		}
	}
}

