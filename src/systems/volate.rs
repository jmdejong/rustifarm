
use specs::{
	Read,
	WriteStorage,
	Entities,
	System,
	Join
};

use crate::{
	components::{Volatile, Removed},
	resources::TimeStamp
};

pub struct Volate;
impl <'a> System<'a> for Volate {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Volatile>,
		WriteStorage<'a, Removed>,
		Read<'a, TimeStamp>
	);
	fn run(&mut self, (entities, mut volatiles, mut removals, timestamp): Self::SystemData) {
		for (ent, volatile) in (&entities, &mut volatiles).join() {
			if let Some(time) = volatile.end_time {
				if time <= timestamp.time {
					removals.insert(ent, Removed).unwrap();
				}
			} else {
				volatile.end_time = Some(timestamp.time + volatile.delay);
			}
		}
	}
}
