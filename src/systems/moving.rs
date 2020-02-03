
use specs::{
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
	Position
};

use super::super::controls::{
	Control
};

use super::super::resources::{
	Size,
	Floor
};



pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (ReadStorage<'a, Controller>, WriteStorage<'a, Position>, Read<'a, Size>, ReadStorage<'a, Blocking>, Read<'a, Floor>);
	fn run(&mut self, (controllers, mut positions, size, blocking, floor): Self::SystemData) {
		for (controller, mut pos) in (&controllers, &mut positions.restrict_mut()).join(){
			match &controller.0 {
				Control::Move(direction) => {
					let newpos = (pos.get_unchecked().pos + direction.to_position()).clamp(Pos::new(0, 0), Pos::new(size.width - 1, size.height - 1));
					let mut blocked = false;
					for ent in floor.cells.get(&newpos).unwrap_or(&Vec::new()) {
						if blocking.get(*ent).is_some(){
							blocked = true;
							break;
						}
					}
					if !blocked {
						let mut pos_mut = pos.get_mut_unchecked();
						pos_mut.prev = Some(pos_mut.pos);
						pos_mut.pos = newpos.clone();
					}
				}
				_ => {}
			}
		}
	}
}
