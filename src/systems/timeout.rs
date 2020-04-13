
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
		Timer,
		TimeOffset,
		TriggerBox
	},
	resources::{Time}
};


pub struct Timeout;
impl <'a> System<'a> for Timeout {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Timer>,
		WriteStorage<'a, TriggerBox>,
		Read<'a, Time>,
		WriteStorage<'a, TimeOffset>
	);
	
	fn run(&mut self, (entities, mut timers, mut triggerboxes, time, mut time_offsets): Self::SystemData) {
		for (entity, timer) in (&entities, &mut timers).join(){
			if timer.target_time == None {
				let creation_time = time.time + time_offsets.get(entity).map(|ct| ct.dtime).unwrap_or(0);
				let mut r = 1.0 - rand::random::<f64>() * timer.spread;
				if rand::random() {
					r = 1.0 / r;
				}
				let duration = (timer.delay as f64 * r + 0.4) as i64;
				timer.target_time = Some(creation_time + duration);
			}
			let target_time = timer.target_time.unwrap();
			if target_time <= time.time {
				if target_time < time.time {
					time_offsets.insert(entity, TimeOffset{dtime: target_time.0 - time.time.0}).unwrap();
				} else {
					time_offsets.remove(entity);
				}
				TriggerBox::add_message(&mut triggerboxes, entity, timer.trigger);
				timer.target_time = None;
			}
		}
	}
}
