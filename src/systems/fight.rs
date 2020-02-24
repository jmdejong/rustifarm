
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
	Attacked,
	Fighter,
	Health
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
		WriteStorage<'a, Attacked>,
		ReadStorage<'a, Fighter>,
		ReadStorage<'a, Health>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut attacked, fighters, healths): Self::SystemData) {
		for (entity, controller, position, fighter) in (&entities, &controllers, &positions, &fighters).join(){
			match &controller.0 {
				Control::Attack(directions) => {
					for direction in directions {
						for ent in ground.cells.get(&(position.pos + direction.to_position())).unwrap_or(&HashSet::new()) {
							if healths.contains(*ent) && *ent != entity {
								Attacked::add_attack(&mut attacked, *ent, fighter.attack.clone());
								break;
							}
						}
					}
				}
				_ => {}
			}
		}
	}
}
