
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	System,
	Entities,
	Join
};

use crate::{
	components::{Health, AttackInbox, Dead, Position, Autofight},
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
		WriteStorage<'a, Dead>,
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Autofight>
	);
	fn run(&mut self, (entities, mut attackeds, mut healths, mut deads, positions, mut new, mut autofighters): Self::SystemData) {
		
		for (entity, attacked, autofighter) in (&entities, &attackeds, &mut autofighters).join() {
			for attack in &attacked.messages {
				if attack.damage > 0 {
					if let Some(attacker) = attack.attacker {
						if healths.contains(attacker) && attacker != entity {
							autofighter.target = Some(attacker);
						}
					}
				}
			}
		}
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
				deads.insert(ent, Dead).unwrap();
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

