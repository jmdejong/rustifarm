
use specs::{
	Write,
	WriteStorage,
	System
};

use crate::{
	resources::Ground,
	components::TriggerBox
};

pub struct Clear;
impl <'a> System<'a> for Clear {
	type SystemData = (
		Write<'a, Ground>,
		WriteStorage<'a, TriggerBox>
	);
	fn run(&mut self, (mut ground, mut triggerboxes): Self::SystemData) {
		ground.changes.clear();
		triggerboxes.clear();
	}
}

