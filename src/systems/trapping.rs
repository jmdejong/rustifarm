
use specs::{
	WriteStorage,
	ReadStorage,
	Entities,
	Read,
	System,
	Join
};

use crate::{
	components::{Health, AttackInbox, Moved, Entered, Trap, Position},
	resources::Ground
};


pub struct Trapping;
impl <'a> System<'a> for Trapping {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, AttackInbox>,
		ReadStorage<'a, Health>,
		ReadStorage<'a, Moved>,
		ReadStorage<'a, Entered>,
		ReadStorage<'a, Trap>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>
	);
	fn run(&mut self, (entities, mut victims, healths, moves, entereds, traps, positions, ground): Self::SystemData) {
		
		for (entity, _entered, trap, position) in (&entities, &entereds, &traps, &positions).join() {
			for ent in ground.cells.get(&position.pos).unwrap(){
				if ent != &entity && moves.contains(*ent) && healths.contains(*ent) {
					AttackInbox::add_message(&mut victims, *ent, trap.attack.clone());
				}
			}
		}
	}
}

