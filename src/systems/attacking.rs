
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	System,
	Entities,
	Join
};

use crate::{
	components::{Health, AttackInbox, Dying, Removed, Position},
	resources::NewEntities,
	Template,
	util
};


pub struct Attacking;
impl <'a> System<'a> for Attacking {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, AttackInbox>,
		WriteStorage<'a, Health>,
		WriteStorage<'a, Dying>,
		WriteStorage<'a, Removed>,
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>
	);
	fn run(&mut self, (entities, mut attackeds, mut healths, mut deads, mut removals, positions, mut new): Self::SystemData) {
		for (ent, health, attacked) in (&entities, &mut healths, &mut attackeds).join() {
			let mut wounded = false;
			for attack in attacked.messages.drain(..) {
				health.health -= attack.damage;
				if attack.damage > 0 {
					wounded = true;
				}
			}
			health.health = util::clamp(health.health, 0, health.maxhealth);
			if health.health == 0 {
				deads.insert(ent, Dying).unwrap();
				removals.insert(ent, Removed).unwrap();
			}
			if let Some(position) = positions.get(ent){
				if wounded {
					new.create(position.pos, Template::empty("wound")).unwrap();
				}
			}
		}
		attackeds.clear();
	}
}

