
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
		OwnTime
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
		ReadStorage<'a, OwnTime>
	);
	
	fn run(&mut self, (positions, mut new, triggerboxes, builds, own_times): Self::SystemData) {
		for (position, triggerbox, build, own_time) in (&positions, &triggerboxes, &builds, (&own_times).maybe()).join(){
			for message in triggerbox.messages.iter() {
				match message {
					Trigger::Build | Trigger::Change => {
						// todo: better error handling
						let mut preent = new.encyclopedia.construct(&build.obj).unwrap();
						if let Some(time) = own_time {
							preent.push(ComponentWrapper::OwnTime(time.clone()));
						}
						new.to_build.push((position.pos, preent));
					}
					_ => {}
				}
			}
		}
	}
}
