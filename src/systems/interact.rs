
use std::collections::HashSet;
use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Read
};

use crate::components::{
	Controller,
	Position,
	ControlCooldown,
	Interactable,
	Dead
};

use crate::controls::{Control};
use crate::resources::{Ground};



pub struct Interact;
impl <'a> System<'a> for Interact {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>,
		WriteStorage<'a, ControlCooldown>,
		ReadStorage<'a, Interactable>,
		WriteStorage<'a, Dead>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut cooldowns, interactables, mut deads): Self::SystemData) {
		for (entity, controller, position) in (&entities, &controllers, &positions).join(){
			let mut target = None;
			match &controller.control {
				Control::Interact(directions) => {
					'targets: for direction in directions {
						for ent in ground.cells.get(&(position.pos + direction.to_position())).unwrap_or(&HashSet::new()) {
							if let Some(interactable) = interactables.get(*ent) {
								target = Some((*ent, interactable));
								break 'targets;
							}
						}
					}
				}
				_ => {}
			}
			if let Some((ent, interactable)) = target {
				match interactable {
					Interactable::Harvest => {
						deads.insert(ent, Dead).unwrap();
					}
				}
				cooldowns.insert(entity, ControlCooldown{amount: 2}).unwrap();
			}
		}
	}
}
