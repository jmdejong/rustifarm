
use specs::{
	ReadStorage,
	System,
	Join,
	Entities
};

use crate::components::{Removed};



pub struct Remove;
impl <'a> System<'a> for Remove {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Removed>
	);
	
	fn run(&mut self, (entities, removals): Self::SystemData) {
		for (ent, _) in (&*entities, &removals).join() {
			if let Err(msg) = entities.delete(ent){
				println!("{:?}", msg);
			}
		}
	}
}
