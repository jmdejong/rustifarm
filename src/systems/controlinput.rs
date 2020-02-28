
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	Entities,
	System,
	Join
};

use crate::components::{Controller, Player, ControlCooldown};
use crate::resources::{Input};


pub struct ControlInput;
impl <'a> System<'a> for ControlInput {
	type SystemData = (
		Entities<'a>,
		Write<'a, Input>,
		WriteStorage<'a, Controller>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, ControlCooldown>
	);
	fn run(&mut self, (entities, mut input, mut controllers, players, cooldowns): Self::SystemData) {
		controllers.clear();
	
		for (player, entity, ()) in (&players, &entities, !&cooldowns).join() {
			if let Some(control) = input.actions.remove(&player.id){
				let _ = controllers.insert(entity, Controller{control: control});
			}
		}
	}
}

