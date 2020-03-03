
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
	AttackMessage,
	Fighter,
	Health,
	ControlCooldown,
	Autofight
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
		WriteStorage<'a, ControlCooldown>,
		WriteStorage<'a, Autofight>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut attacked, fighters, healths, mut cooldowns, mut autofighters): Self::SystemData) {
		for (entity, controller, position, fighter) in (&entities, &controllers, &positions, &fighters).join(){
			let mut target = None;
			match &controller.control {
				Control::Attack(directions) => {
					'targets: for direction in directions {
						for ent in ground.cells.get(&(position.pos + direction.to_position())).unwrap_or(&HashSet::new()) {
							if healths.contains(*ent) && *ent != entity {
								target = Some(*ent);
								break 'targets;
							}
						}
					}
				}
				Control::AttackTarget(t) => {
					if *t == entity { // don't knock yourself out
					} else if let Some(target_position) = positions.get(*t){
						if position.pos.distance_to(target_position.pos) <= fighter.range {
							target = Some(*t);
						}
					}
				}
				_ => {}
			}
			if let Some(ent) = target {
				AttackInbox::add_message(&mut attacked, ent, AttackMessage{typ: fighter.attack.clone(), attacker: Some(entity)});
				cooldowns.insert(entity, ControlCooldown{amount: fighter.cooldown}).unwrap();
				if let Some(autofighter) = autofighters.get_mut(entity){
					autofighter.target = Some(ent);
				}
			}
		}
	}
}
