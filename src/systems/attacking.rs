
use specs::{
	WriteStorage,
	System,
	Entities,
	Join
};

use crate::{
	components::{Health, Attacked, Dying, Removed},
	util
};


pub struct Attacking;
impl <'a> System<'a> for Attacking {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Attacked>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, Dying>,
		WriteStorage<'a, Removed>
	);
	fn run(&mut self, (entities, mut victims, mut healths, mut deads, mut removals): Self::SystemData) {
		for (ent, health, attacked) in (&entities, &mut healths, &mut victims).join() {
			for attack in attacked.attacks.drain(..) {
				health.health -= attack.damage;
			}
			health.health = util::clamp(health.health, 0, health.maxhealth);
			if health.health == 0 {
				deads.insert(ent, Dying).unwrap();
				removals.insert(ent, Removed).unwrap();
			}
		}
		victims.clear();
	}
}

