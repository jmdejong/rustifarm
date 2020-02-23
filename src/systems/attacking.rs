
use specs::{
	WriteStorage,
	System,
	Join
};

use crate::{
	components::{Health, Attacked},
	util
};


pub struct Attacking;
impl <'a> System<'a> for Attacking {
	type SystemData = (
		WriteStorage<'a, Attacked>,
		WriteStorage<'a, Health>
	);
	fn run(&mut self, (mut victims, mut healths): Self::SystemData) {
		
		for (health, attacked) in (&mut healths, &mut victims).join() {
			for attack in attacked.attacks.drain(..) {
				health.health -= attack.damage;
			}
			health.health = util::clamp(health.health, 0, health.maxhealth);
		}
	}
}

