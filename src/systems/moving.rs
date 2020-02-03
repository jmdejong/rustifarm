
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
	Blocking
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
	type SystemData = (ReadStorage<'a, Controller>, WriteStorage<'a, Pos>, Read<'a, Size>, ReadStorage<'a, Blocking>, Read<'a, Floor>);
	fn run(&mut self, (controller, mut pos, size, blocking, floor): Self::SystemData) {
		for (controller, pos) in (&controller, &mut pos).join(){
			match &controller.0 {
				Control::Move(direction) => {
					let newpos = (*pos + direction.to_position()).clamp(Pos::new(0, 0), Pos::new(size.width - 1, size.height - 1));
					let mut blocked = false;
					for ent in floor.cells.get(&newpos).unwrap_or(&Vec::new()) {
						if blocking.get(*ent).is_some(){
							blocked = true;
							break;
						}
					}
					if !blocked {
						pos.clone_from(&newpos);
					}
				}
				_ => {}
			}
		}
	}
}
