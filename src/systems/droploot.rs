
use rand::Rng;

use specs::{
	ReadStorage,
	System,
	Join,
	Write
};

use crate::{
	components::{
		Position,
		Loot,
		Trigger,
		TriggerBox
	},
	resources::{NewEntities}
};


pub struct DropLoot;
impl <'a> System<'a> for DropLoot{
	type SystemData = (
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		ReadStorage<'a, TriggerBox>,
		ReadStorage<'a, Loot>
	);
	
	fn run(&mut self, (positions, mut new, triggerboxes, loots): Self::SystemData) {
		for (position, triggerbox, loot) in (&positions, &triggerboxes, &loots).join(){
			for message in triggerbox.messages.iter() {
				match message {
					Trigger::Die | Trigger::Loot => {
						for (template, chance) in &loot.loot {
							if *chance > rand::thread_rng().gen_range(0.0, 1.0) {
								// todo: better error handling
								new.create(position.pos, &template).unwrap();
							}
						}
					}
					_ => {}
				}
			}
		}
	}
}
