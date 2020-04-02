

use rand::Rng;
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	System,
	Entities,
	Join
};

use crate::{
	components::{Health, AttackInbox, AttackType, Dead, Position, Autofight},
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
				if attack.typ.is_hostile() {
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
				match attack.typ {
					AttackType::Attack(strength) => {
						let damage = rand::thread_rng().gen_range(0, strength+1);
						health.health -= damage;
						if damage > 0 {
							wounded = true;
						}
					}
					AttackType::Heal(healthdiff) => {
						health.health += healthdiff;
					}
				}
			}
			health.health = util::clamp(health.health, 0, health.maxhealth);
			if health.health == 0 {
				deads.insert(ent, Dead).unwrap();
			}
			if let Some(position) = positions.get(ent){
				if wounded {
					new.create(position.pos, &Template::empty("wound")).unwrap();
				}
			}
		}
		attackeds.clear();
	}
}

