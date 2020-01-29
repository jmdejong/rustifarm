
use specs::{
	ReadStorage,
	WriteStorage,
	Read,
	Write,
	Entities,
	System,
	Join
};

use super::components::{
	Position,
	Visible,
	Controller,
	Blocking
};

use super::controls::Control;

use super::resources::{
	TopView,
	Size,
	Floor
};


pub struct MakeFloor;
impl <'a> System<'a> for MakeFloor {
	type SystemData = (Entities<'a>, Write<'a, Floor>, ReadStorage<'a, Position>);
	fn run(&mut self, (entities, mut floor, positions): Self::SystemData) {
		floor.cells.clear();
		for (ent, pos) in (&entities, &positions).join() {
			floor.cells.entry(*pos).or_insert(Vec::new()).push(ent);
		}
	}
}

pub struct Draw;
impl <'a> System<'a> for Draw {
	
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Write<'a, TopView>);
	
	fn run(&mut self, (pos, vis, mut view): Self::SystemData) {
		view.cells.clear();
		for (pos, vis) in (&pos, &vis).join(){
			view.cells.entry(*pos).or_insert(Vec::new()).push(vis.clone());
			view.cells.get_mut(pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
		}
	}
}

pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (ReadStorage<'a, Controller>, WriteStorage<'a, Position>, Read<'a, Size>, ReadStorage<'a, Blocking>, Read<'a, Floor>);
	fn run(&mut self, (controller, mut pos, size, blocking, floor): Self::SystemData) {
		for (controller, pos) in (&controller, &mut pos).join(){
			match &controller.0 {
				Control::Move(direction) => {
					let newpos = (*pos + direction.to_position()).clamp(Position::new(0, 0), Position::new(size.width - 1, size.height - 1));
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


pub struct ClearControllers;
impl <'a> System<'a> for ClearControllers {
	type SystemData = (Entities<'a>, WriteStorage<'a, Controller>);
	fn run(&mut self, (entities, mut controllers): Self::SystemData) {
		let mut ents = Vec::new();
		for (ent, _controller) in (&*entities, &controllers).join() {
			ents.push(ent);
		}
		for ent in ents {
			controllers.remove(ent);
		}
	}
}

