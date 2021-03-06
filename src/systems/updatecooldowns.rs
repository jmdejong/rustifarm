
use specs::{
	WriteStorage,
	Entities,
	System,
	Join
};

use crate::components::{Controller, ControlCooldown};


pub struct UpdateCooldowns;
impl <'a> System<'a> for UpdateCooldowns {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, ControlCooldown>,
		WriteStorage<'a, Controller>
	);
	fn run(&mut self, (entities, mut cooldowns, mut controllers): Self::SystemData) {
		controllers.clear();
		let mut to_remove = Vec::new();
		for (entity, cooldown) in (&entities, &mut cooldowns).join() {
			if cooldown.amount > 0 {
				cooldown.amount -= 1;
			}
			if cooldown.amount <= 0 {
				to_remove.push(entity);
			}
		}
		for entity in to_remove {
			cooldowns.remove(entity);
		}
	}
}

