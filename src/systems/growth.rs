
use rand;

use specs::{
	WriteStorage,
	Entities,
	System,
	Join,
	Read
};

use crate::{
	components::{
		Grow,
		OwnTime,
		TriggerBox
	},
	resources::{Time}
};


pub struct Growth;
impl <'a> System<'a> for Growth{
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Grow>,
		WriteStorage<'a, TriggerBox>,
		Read<'a, Time>,
		WriteStorage<'a, OwnTime>
	);
	
	fn run(&mut self, (entities, mut grows, mut triggerboxes, time, mut own_times): Self::SystemData) {
		for (entity, grow) in (&entities, &mut grows).join(){
			if grow.target_time == None {
				let creation_time = own_times.get(entity).map(|ct| ct.time).unwrap_or(time.time);
				let duration = grow.delay as f64 * (1.0 + rand::random::<f64>()) / (if rand::random() {1.0} else {2.0});
				grow.target_time = Some(creation_time + duration as i64);
			}
			let target_time = grow.target_time.unwrap();
			if target_time <= time.time {
				if target_time + 1 < time.time {
					own_times.insert(entity, OwnTime{time: target_time + 1}).unwrap();
				}
				TriggerBox::add_message(&mut triggerboxes, entity, grow.trigger);
			}
		}
	}
}
