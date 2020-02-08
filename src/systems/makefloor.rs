
use std::collections::HashSet;

use specs::{
	ReadStorage,
	Write,
	Entities,
	System,
	Join
};

use crate::components::{
	Position,
	New,
	Moved,
	Removed
};

use crate::resources::{
	Ground
};


#[derive(Default)]
pub struct MakeFloor;
impl <'a> System<'a> for MakeFloor {
	type SystemData = (
		Entities<'a>,
		Write<'a, Ground>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, New>,
		ReadStorage<'a, Moved>,
		ReadStorage<'a, Removed>
	);
	fn run(&mut self, (entities, mut ground, positions, new, moved, removed): Self::SystemData) {
		for (ent, pos, _new) in (&entities, &positions, &new).join() {
			ground.cells.entry(pos.pos).or_insert(HashSet::new()).insert(ent);
		}
		for (ent, pos, mov) in (&entities, &positions, &moved).join() {
			ground.cells.entry(pos.pos).or_insert(HashSet::new()).insert(ent);
			ground.cells.get_mut(&mov.from).unwrap().remove(&ent);
		}
		for (ent, pos, _removed) in (&entities, &positions, &removed).join() {
			ground.cells.get_mut(&pos.pos).unwrap().remove(&ent);
		}
	}
}

