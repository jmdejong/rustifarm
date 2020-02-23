
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
		Blocking,
		Position,
		Floor,
		Moved
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
		ReadStorage<'a, Blocking>,
		Write<'a, Ground>,
		ReadStorage<'a, Floor>,
		WriteStorage<'a, Moved>
	);
	
	fn run(&mut self, (entities, controllers, mut positions, size, blocking, mut ground, floor, mut moved): Self::SystemData) {
		{
			let mut ents = Vec::new();
			for (ent, _moved) in (&*entities, &moved).join() {
				ents.push(ent);
			}
			for ent in ents {
				moved.remove(ent);
			}
		}
		for (ent, controller, mut pos) in (&entities, &controllers, &mut positions.restrict_mut()).join(){
			match &controller.0 {
				Control::Move(direction) => {
					let newpos = (pos.get_unchecked().pos + direction.to_position()).clamp(Pos::new(0, 0), Pos::new(size.width - 1, size.height - 1));
					let mut blocked = false;
					let mut on_floor = false;
					for ent in ground.cells.get(&newpos).unwrap_or(&HashSet::new()) {
						if blocking.get(*ent).is_some(){
							blocked = true;
							break;
						}
						if floor.get(*ent).is_some(){
							on_floor = true;
						}
					}
					if !blocked && on_floor {
						let mut pos_mut = pos.get_mut_unchecked();
						moved.insert(ent, Moved{from: pos_mut.pos}).expect("can't insert Moved");
						ground.cells.get_mut(&pos_mut.pos).unwrap().remove(&ent);
						pos_mut.pos = newpos;
						ground.cells.entry(newpos).or_insert_with(HashSet::new).insert(ent);
					}
				}
				_ => {}
			}
		}
	}
}
