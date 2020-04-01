
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
		Removed,
		CreationTime
	},
	resources::{NewEntities, Time},
	componentwrapper::ComponentWrapper
};


pub struct Growth;
impl <'a> System<'a> for Growth{
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Grow>,
		WriteStorage<'a, Removed>,
		Read<'a, Time>,
		ReadStorage<'a, CreationTime>
	);
	
	fn run(&mut self, (entities, positions, mut new, mut grows, mut removeds, time, creation_times): Self::SystemData) {
		for (entity, position, grow) in (&entities, &positions, &mut grows).join(){
			let creation_time = creation_times.get(entity).map(|ct| ct.time).unwrap_or(time.time);
			if grow.target_time == None {
				let duration = grow.delay as f64 * (1.0 + rand::random::<f64>()) / (if rand::random() {1.0} else {2.0});
				grow.target_time = Some(creation_time + duration as i64);
			}
			let target_time = grow.target_time.unwrap();
			if target_time <= time.time {
				removeds.insert(entity, Removed).unwrap();
				// todo: error handling
				let mut preent = new.encyclopedia.construct(&grow.into).unwrap();
				preent.push(ComponentWrapper::CreationTime(CreationTime{time: target_time + 1}));
				new.to_build.push((position.pos, preent));
			}
		}
	}
}
