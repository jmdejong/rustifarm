
use std::collections::HashSet;
use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Read,
	Write
};

use crate::components::{
	Controller,
	Position,
	ControlCooldown,
	Interactable,
	Dead,
	Removed
};

use crate::controls::{Control};
use crate::resources::{Ground, NewEntities};



pub struct Interact;
impl <'a> System<'a> for Interact {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>,
		WriteStorage<'a, ControlCooldown>,
		ReadStorage<'a, Interactable>,
		WriteStorage<'a, Dead>,
		WriteStorage<'a, Removed>,
		Write<'a, NewEntities>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut cooldowns, interactables, mut deads, mut removeds, mut new): Self::SystemData) {
		for (entity, controller, position) in (&entities, &controllers, &positions).join(){
			let mut target = None;
			match &controller.control {
				Control::Interact(directions) => {
					'targets: for direction in directions {
						let pos = position.pos + direction.to_position();
						for ent in ground.cells.get(&pos).unwrap_or(&HashSet::new()) {
							if let Some(interactable) = interactables.get(*ent) {
								target = Some((*ent, interactable, pos));
								break 'targets;
							}
						}
					}
				}
				_ => {}
			}
			if let Some((ent, interactable, pos)) = target {
				match interactable {
					Interactable::Harvest => {
						deads.insert(ent, Dead).unwrap();
					}
					Interactable::Change(into) => {
						new.create(pos, into).unwrap();
						removeds.insert(ent, Removed).unwrap();
					}
				}
				cooldowns.insert(entity, ControlCooldown{amount: 2}).unwrap();
			}
		}
	}
}
