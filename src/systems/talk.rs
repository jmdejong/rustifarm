

use specs::{
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Read
};

use crate::{
	components::{
		Controller,
		Position,
		Talkable,
		Notification,
		Ear,
		Visible
	},
	controls::{Control},
	resources::{Ground},
};

pub struct Talk;
impl <'a> System<'a> for Talk {
	type SystemData = (
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>,
		ReadStorage<'a, Talkable>,
		WriteStorage<'a, Ear>,
		ReadStorage<'a, Visible>
	);
	
	fn run(&mut self, (controllers, positions, ground, talkables, mut ears, visibles): Self::SystemData) {
		for (controller, position, ear) in (&controllers, &positions, &mut ears).join(){
			match &controller.control {
				Control::Interact(directions, None) => {
					for (ent, Talkable{text}) in ground.components_near(position.pos, directions, &talkables) {
						let name = visibles.get(ent).map(|v| v.name.clone());
						ear.sounds.push(Notification::Sound{text: text.clone(), source: name});
						break;
					}
				}
				_ => {}
			}
		}
	}
}

