
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
	AttackInbox,
	Fighter,
	Health,
	ControlCooldown
};

use crate::controls::{Control};
use crate::resources::{Ground};



pub struct Fight;
impl <'a> System<'a> for Fight {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		Read<'a, Ground>,
		WriteStorage<'a, AttackInbox>,
		ReadStorage<'a, Fighter>,
		ReadStorage<'a, Health>,
		WriteStorage<'a, ControlCooldown>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut attacked, fighters, healths, mut cooldowns): Self::SystemData) {
		for (entity, controller, position, fighter) in (&entities, &controllers, &positions, &fighters).join(){
			match &controller.control {
				Control::Attack(directions) => {
					'targets: for direction in directions {
						for ent in ground.cells.get(&(position.pos + direction.to_position())).unwrap_or(&HashSet::new()) {
							if healths.contains(*ent) && *ent != entity {
								AttackInbox::add_message(&mut attacked, *ent, fighter.attack.clone());
								cooldowns.insert(entity, ControlCooldown{amount: fighter.cooldown}).unwrap();
								break 'targets;
							}
						}
					}
				}
				_ => {}
			}
		}
	}
}
