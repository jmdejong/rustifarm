
use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use crate::{
	components::{
		Controller,
		Position,
		Flags,
		Flag,
		Moved,
		Entered,
		Movable,
		ControlCooldown
	},
	controls::{
		Control
	},
	resources::{
		Ground
	},
};


pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		ReadStorage<'a, Flags>,
		Write<'a, Ground>,
		WriteStorage<'a, Moved>,
		WriteStorage<'a, Entered>,
		ReadStorage<'a, Movable>,
		WriteStorage<'a, ControlCooldown>
	);
	
	fn run(&mut self, (entities, controllers, mut positions, flags, mut ground, mut moved, mut entered, movables, mut cooldowns): Self::SystemData) {
		moved.clear();
		entered.clear();
		for (ent, controller, mut position, movable) in (&entities, &controllers, &mut positions, &movables).join(){
			match &controller.control {
				Control::Move(direction) => {
					let newpos = position.pos + direction.to_position();
					let ground_flags = ground.flags_on(newpos, &flags);
					if !ground_flags.contains(&Flag::Blocking) && ground_flags.contains(&Flag::Floor) {
						moved.insert(ent, Moved{from: position.pos}).expect("can't insert Moved");
						ground.remove(&position.pos, ent);
						position.pos = newpos;
						ground.insert(newpos, ent);
						for ent in ground.cells.get(&newpos).unwrap() {
							let _ = entered.insert(*ent, Entered);
						}
						cooldowns.insert(ent, ControlCooldown{amount: movable.cooldown}).unwrap();
					}
				}
				_ => {}
			}
		}
	}
}
