
use std::collections::HashSet;

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	Read,
	System,
	Join
};

use super::super::pos::Pos;

use super::super::components::{
	Controller,
	Blocking,
	Position,
	Floor,
	Moved
};

use super::super::controls::{
	Control
};

use super::super::resources::{
	Size,
	Ground
};



pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		Read<'a, Size>,
		ReadStorage<'a, Blocking>,
		Read<'a, Ground>,
		ReadStorage<'a, Floor>,
		WriteStorage<'a, Moved>
	);
	
	fn run(&mut self, (entities, controllers, mut positions, size, blocking, ground, floor, mut moved): Self::SystemData) {
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
						pos_mut.prev = Some(pos_mut.pos);
						moved.insert(ent, Moved{from: pos_mut.pos}).expect("can't insert Moved");
						pos_mut.pos = newpos.clone();
					}
				}
				_ => {}
			}
		}
	}
}
