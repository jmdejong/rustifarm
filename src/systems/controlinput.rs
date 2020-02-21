
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	Entities,
	System,
	Join
};

use crate::components::{Controller, Player};
use crate::resources::{Input};


pub struct ControlInput;
impl <'a> System<'a> for ControlInput {
	type SystemData = (
		Entities<'a>,
		Write<'a, Input>,
		WriteStorage<'a, Controller>,
		ReadStorage<'a, Player>
	);
	fn run(&mut self, (entities, mut input, mut controllers, players): Self::SystemData) {
		{
			let mut ents = Vec::new();
			for (ent, _controller) in (&*entities, &controllers).join() {
				ents.push(ent);
			}
			for ent in ents {
				controllers.remove(ent);
			}
		}
	
		for (player, entity) in (&players, &entities).join() {
			if let Some(control) = input.actions.get(&player.id){
				let _ = controllers.insert(entity, Controller(control.clone()));
			}
		}
		input.actions.clear();
	}
}

