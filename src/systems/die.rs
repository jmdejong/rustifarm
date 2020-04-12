
use specs::{
	Write,
	WriteStorage,
	ReadStorage,
	Entities,
	System,
	Join
};

use crate::{
	components::{Trigger, TriggerBox, Removed, Player},
	resources::Emigration,
	purgatory,
	playerstate::RoomPos
};


pub struct Die;
impl <'a> System<'a> for Die {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, TriggerBox>,
		WriteStorage<'a, Removed>,
		Write<'a, Emigration>,
		ReadStorage<'a, Player>
	);
	fn run(&mut self, (entities, triggerboxes, mut removeds, mut emigration, players): Self::SystemData) {
		for (entity, triggerbox) in (&entities, &triggerboxes).join() {
			for trigger in triggerbox.messages.iter() {
				match trigger {
					Trigger::Die | Trigger::Remove => {
						if let Some(player) = players.get(entity) {
							// players move to purgatory when dead
							emigration.emigrants.push((player.id.clone(), purgatory::purgatory_id(), RoomPos::Unknown));
						} else {
							// npcs etc get removed when dead
							removeds.insert(entity, Removed).unwrap();
						}
					}
					_ => {}
				}
			}
		}
	}
}

