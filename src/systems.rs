
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	System,
	Join
};

use super::components::{
	Position,
	Visible,
	Controller
};

use super::controls::Control;

use super::resources::TopView;

pub struct Draw;

impl <'a> System<'a> for Draw {
	
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Write<'a, TopView>);
	
	fn run(&mut self, (pos, vis, mut view): Self::SystemData) {
		view.cells.clear();
		for (pos, vis) in (&pos, &vis).join(){
			if pos.x >= 0 && pos.y >= 0 && pos.x < view.width && pos.y < view.height {
				view.cells.entry(*pos).or_insert(Vec::new()).push(vis.clone());
				view.cells.get_mut(pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
			}
		}
	}
}

// struct Control;
// impl <'a> System <'a> for Control {
// 	type SystemData = WriteStorage<'a, Controller>;
// 	fn run (&mut self, mut controller: Self::SystemData) {
// 		for controller in &mut controller.join()
// 	}
// }

pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (WriteStorage<'a, Controller>, WriteStorage<'a, Position>);
	fn run(&mut self, (mut controller, mut pos): Self::SystemData) {
		for (controller, pos) in (&mut controller, &mut pos).join(){
			if let Some(control) = &controller.0 {
				match control {
					Control::Move(direction) => {
						let (dx, dy) = direction.to_position();
						pos.x += dx;
						pos.y += dy;
					}
					_ => {}
				}
				controller.0 = None
			}
		}
	}
}
