
use specs::{
	WriteStorage,
	Read,
	System,
	Join
};

use crate::{
	components::{Health, Healing},
	resources::Time
};


pub struct Heal;
impl <'a> System<'a> for Heal {
	type SystemData = (
		WriteStorage<'a, Health>,
		WriteStorage<'a, Healing>,
		Read<'a, Time>
	);
	fn run(&mut self, (mut healths, mut healing, time): Self::SystemData) {
		for (health, mut heal) in (&mut healths, &mut healing).join() {
			
			if let Some(next_heal) = heal.next_heal {
				if next_heal <= time.time {
					health.heal(heal.health);
					heal.next_heal = None
				}
			}
			if health.health < health.maxhealth && heal.next_heal == None {
				heal.next_heal = Some(time.time + heal.delay)
			}
		}
	}
}

