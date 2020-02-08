
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
	New
};

use crate::resources::{
	Ground
};


#[derive(Default)]
pub struct RegisterNew;
impl <'a> System<'a> for RegisterNew {
	type SystemData = (
		Entities<'a>,
		Write<'a, Ground>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, New>,
	);
	fn run(&mut self, (entities, mut ground, positions, new): Self::SystemData) {
		for (ent, pos, _new) in (&entities, &positions, &new).join() {
			ground.cells.entry(pos.pos).or_insert(HashSet::new()).insert(ent);
		}
	}
}

