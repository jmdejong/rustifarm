
use specs::{
	ReadStorage,
	System,
	Join,
	Write,
	Entities
};

use crate::components::{Removed, Position};
use crate::resources::Ground;


pub struct Remove;
impl <'a> System<'a> for Remove {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Removed>,
		ReadStorage<'a, Position>,
		Write<'a, Ground>
	);
	
	fn run(&mut self, (entities, removals, positions, mut ground): Self::SystemData) {
		for (ent, _) in (&*entities, &removals, ).join() {
			if let Err(msg) = entities.delete(ent){
				println!("{:?}", msg);
			}
			if let Some(position) = positions.get(ent) {
				ground.remove(&position.pos, ent);
			}
		}
	}
}
