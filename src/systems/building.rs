
use specs::{
	ReadStorage,
	System,
	Join,
	Write
};

use crate::{
	components::{
		Position,
		Build,
		Trigger,
		TriggerBox
	},
	resources::{NewEntities}
};


pub struct Building;
impl <'a> System<'a> for Building{
	type SystemData = (
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		ReadStorage<'a, TriggerBox>,
		ReadStorage<'a, Build>
	);
	
	fn run(&mut self, (positions, mut new, triggerboxes, builds): Self::SystemData) {
		for (position, triggerbox, build) in (&positions, &triggerboxes, &builds).join(){
			for message in triggerbox.messages.iter() {
				match message {
					Trigger::Build | Trigger::Change => {
						// todo: better error handling
						new.create(position.pos, &build.obj).unwrap();
					}
					_ => {}
				}
			}
		}
	}
}
