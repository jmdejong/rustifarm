
use specs::{
	ReadStorage,
	System,
	Join,
	Write
};

use crate::{
	ComponentWrapper,
	components::{
		Position,
		Build,
		Trigger,
		TriggerBox,
		TimeOffset
	},
	resources::{NewEntities},
};


pub struct Building;
impl <'a> System<'a> for Building{
	type SystemData = (
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		ReadStorage<'a, TriggerBox>,
		ReadStorage<'a, Build>,
		ReadStorage<'a, TimeOffset>
	);
	
	fn run(&mut self, (positions, mut new, triggerboxes, builds, time_offsets): Self::SystemData) {
		for (position, triggerbox, build, time_offset) in (&positions, &triggerboxes, &builds, (&time_offsets).maybe()).join(){
			for message in triggerbox.messages.iter() {
				match message {
					Trigger::Build | Trigger::Change => {
						// todo: better error handling
						let mut preent = new.encyclopedia.construct(&build.obj).unwrap();
						if let Some(time) = time_offset {
							preent.push(ComponentWrapper::TimeOffset(time.clone()));
						}
						new.to_build.push((position.pos, preent));
					}
					_ => {}
				}
			}
		}
	}
}
