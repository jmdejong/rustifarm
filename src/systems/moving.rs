
use std::collections::HashSet;

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	Read,
	System,
	Join,
	Write
};

use crate::{
	Pos,
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
		Size,
		Ground
	},
};


pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		Read<'a, Size>,
		ReadStorage<'a, Flags>,
		Write<'a, Ground>,
		WriteStorage<'a, Moved>,
		WriteStorage<'a, Entered>,
		ReadStorage<'a, Movable>,
		WriteStorage<'a, ControlCooldown>
	);
	
	fn run(&mut self, (entities, controllers, mut positions, size, flags, mut ground, mut moved, mut entered, movables, mut cooldowns): Self::SystemData) {
		moved.clear();
		entered.clear();
		for (ent, controller, mut pos, movable) in (&entities, &controllers, &mut positions.restrict_mut(), &movables).join(){
			match &controller.control {
				Control::Move(direction) => {
					let newpos = (pos.get_unchecked().pos + direction.to_position()).clamp(Pos::new(0, 0), Pos::new(size.width - 1, size.height - 1));
					let ground_flags = ground.flags_on(newpos, &flags);
					if !ground_flags.contains(&Flag::Blocking) && ground_flags.contains(&Flag::Floor) {
						let mut pos_mut = pos.get_mut_unchecked();
						moved.insert(ent, Moved{from: pos_mut.pos}).expect("can't insert Moved");
						ground.cells.get_mut(&pos_mut.pos).unwrap().remove(&ent);
						pos_mut.pos = newpos;
						ground.cells.entry(newpos).or_insert_with(HashSet::new).insert(ent);
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
