
use specs::{
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Read
};

use crate::components::{
	Controller,
	Position,
	Visible,
	Ear,
	ear::Notification,
};

use crate::controls::{Control};
use crate::resources::{Ground};



pub struct Describe;
impl <'a> System<'a> for Describe {
	type SystemData = (
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Visible>,
		Read<'a, Ground>,
		WriteStorage<'a, Ear>,
	);
	
	fn run(&mut self, (controllers, positions, visibles, ground, mut ears): Self::SystemData) {
		for (controller, position, ear) in (&controllers, &positions, &mut ears).join(){
			match &controller.control {
				Control::Describe(direction) => {
					for entity in ground.by_height(&(position.pos + direction.to_position()), &visibles) {
						let visible = visibles.get(entity).unwrap();
						let name = visible.name.clone();
						let description = visible.description.clone().unwrap_or("".to_string());
						ear.sounds.push(Notification::Describe{name, description});
					}
				}
				_ => {}
			}
		}
	}
	
}
