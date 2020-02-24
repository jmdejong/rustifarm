
use specs::{
	WriteStorage,
	Read,
	System,
	Join
};

use crate::{
	components::{Health, Healing},
	resources::TimeStamp
};


pub struct Heal;
impl <'a> System<'a> for Heal {
	type SystemData = (
		WriteStorage<'a, Health>,
		WriteStorage<'a, Healing>,
		Read<'a, TimeStamp>
	);
	fn run(&mut self, (mut healths, mut healing, timestamp): Self::SystemData) {
		for (health, mut heal) in (&mut healths, &mut healing).join() {
			
			if let Some(next_heal) = heal.next_heal {
				if next_heal <= timestamp.time {
					health.heal(heal.health);
					heal.next_heal = None
				}
			}
			if health.health < health.maxhealth && heal.next_heal == None {
				heal.next_heal = Some(timestamp.time + heal.delay)
			}
		}
	}
}

