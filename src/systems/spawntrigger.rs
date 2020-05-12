

use specs::{
	WriteStorage,
	ReadStorage,
	Entities,
	System,
	Join
};

use crate::{
	components::{
		TriggerBox,
		OnSpawn,
		New
	}
};


pub struct SpawnTrigger;
impl <'a> System<'a> for SpawnTrigger {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, TriggerBox>,
		ReadStorage<'a, OnSpawn>,
		ReadStorage<'a, New>
	);
	
	fn run(&mut self, (entities, mut triggerboxes, onspawns, news): Self::SystemData) {
		for (entity, onspawn, _new) in (&entities, &onspawns, &news).join(){
			TriggerBox::add_message(&mut triggerboxes, entity, onspawn.trigger);
		}
	}
}
