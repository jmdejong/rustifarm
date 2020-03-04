
use rand;

use specs::{
	ReadStorage,
	WriteStorage,
	Entities,
	System,
	Join,
	Write,
	Read
};

use crate::{
	components::{
		Position,
		Grow,
		Removed
	},
	resources::{NewEntities, Time}
};


pub struct Growth;
impl <'a> System<'a> for Growth{
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Grow>,
		WriteStorage<'a, Removed>,
		Read<'a, Time>
	);
	
	fn run(&mut self, (entities, positions, mut new, mut grows, mut removeds, time): Self::SystemData) {
		for (entity, position, grow) in (&entities, &positions, &mut grows).join(){
			if grow.target_time == None {
				let duration = grow.delay as f64 * (1.0 + rand::random::<f64>()) / (if rand::random() {1.0} else {2.0});
				grow.target_time = Some(time.time + duration as i64);
			}
			if grow.target_time.unwrap() <= time.time {
				removeds.insert(entity, Removed).unwrap();
				// todo: error handling
				new.create(position.pos, grow.into.clone()).unwrap();
			}
		}
	}
}
