
use std::collections::HashSet;

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
					'targets: for direction in directions {
						let pos = position.pos + direction.to_position();
						for ent in ground.cells.get(&pos).unwrap_or(&HashSet::new()) {
							if let Some(Talkable{text}) = talkables.get(*ent) {
								let name = visibles.get(*ent).map(|v| v.name.clone());
								ear.sounds.push(Notification::Sound{text: text.clone(), source: name});
								break 'targets;
							}
						}
					}
				}
				_ => {}
			}
		}
	}
}

