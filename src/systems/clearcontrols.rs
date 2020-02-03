
use specs::{
	WriteStorage,
	Entities,
	System,
	Join
};

use super::super::components::Controller;

pub struct ClearControllers;
impl <'a> System<'a> for ClearControllers {
	type SystemData = (Entities<'a>, WriteStorage<'a, Controller>);
	fn run(&mut self, (entities, mut controllers): Self::SystemData) {
		let mut ents = Vec::new();
		for (ent, _controller) in (&*entities, &controllers).join() {
			ents.push(ent);
		}
		for ent in ents {
			controllers.remove(ent);
		}
	}
}


