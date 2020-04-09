

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
	components::{
		Health,
		AttackInbox,
		AttackType,
		Dead,
		Position,
		Autofight,
		Ear,
		ear::{Notification, HealthNotification::{Attack, Damage, Heal}, say},
		Visible
	},
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
		WriteStorage<'a, Autofight>,
		WriteStorage<'a, Ear>,
		ReadStorage<'a, Visible>
	);
	fn run(&mut self, (entities, mut attackeds, mut healths, mut deads, positions, mut new, mut autofighters, mut ears, visibles): Self::SystemData) {
		
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
		for (target, health, attacked) in (&entities, &mut healths, &mut attackeds).join() {
			let target_name = visibles.get(target).map(|v| v.name.as_str()).unwrap_or("?").to_string();
			let mut wounded = false;
			let mut attackers = Vec::new();
			let mut attacker_names = Vec::new();
			for attack in attacked.messages.drain(..) {
				let actor_name = attack.attacker.map(|ae| visibles.get(ae)).flatten().map(|v| v.name.as_str()).unwrap_or("?").to_string();
				match attack.typ {
					AttackType::Attack(strength) => {
						let damage = rand::thread_rng().gen_range(0, strength+1);
						health.health -= damage;
						if damage > 0 {
							wounded = true;
							if let Some(actor) = attack.attacker {
								say(&mut ears, actor, Notification::Health{actor: actor_name.clone(), target: target_name.clone(), amount: damage, typ: Attack});
								attackers.push(actor);
								attacker_names.push(actor_name.clone());
							}
							say(&mut ears, target, Notification::Health{actor: actor_name.clone(), target: target_name.clone(), amount: damage, typ: Damage});
						}
					}
					AttackType::Heal(healthdiff) => {
						say(&mut ears, target, Notification::Health{actor: actor_name.clone(), target: target_name.clone(), amount: healthdiff, typ: Heal});
						health.health += healthdiff;
					}
				}
			}
			health.health = util::clamp(health.health, 0, health.maxhealth);
			if health.health == 0 {
				deads.insert(target, Dead).unwrap();
				let killers = attacker_names.join(" and ");
				say(&mut ears, target, Notification::Die{actor: killers.clone(), target: target_name.clone()});
				for actor in attackers {
					say(&mut ears, actor, Notification::Kill{actor: killers.clone(), target: target_name.clone()});
				}
			}
			if let Some(position) = positions.get(target){
				if wounded {
					new.create(position.pos, &Template::empty("wound")).unwrap();
				}
			}
		}
		attackeds.clear();
	}
}

