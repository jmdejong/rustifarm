
use std::collections::HashMap;
use rand;
use rand::Rng;

use specs::{
	WriteStorage,
	ReadStorage,
	Write,
	System,
	Join,
	Entities
};

use crate::{
	components::{
		Position,
		Spawner,
		Clan,
		Home,
		TriggerBox,
		Trigger,
		TimeOffset
	},
	resources::{NewEntities},
	componentwrapper::ComponentWrapper,
	Pos,
	fromtoparameter::FromToParameter
};



pub struct Spawn;
impl <'a> System<'a> for Spawn {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Spawner>,
		ReadStorage<'a, Clan>,
		ReadStorage<'a, TriggerBox>,
		WriteStorage<'a, TimeOffset>
	);
	
	fn run(&mut self, (entities, positions, mut new, mut spawners, clans, triggerboxes, mut timeoffsets): Self::SystemData) {
		let mut clan_nums: HashMap<&Clan, usize> = HashMap::new();
		for clan in (&clans).join() {
			let n: usize = *clan_nums.entry(clan).or_insert(0);
			clan_nums.insert(clan, n+1);
		}
		let mut rng = rand::thread_rng();
		for (entity, spawner, position, triggerbox) in (&entities, &mut spawners, &positions, &triggerboxes).join() {
			if spawner.clan.name == "" {
				spawner.clan.name = format!("$random({},{},{})", position.pos.x, position.pos.y, spawner.template.name.0);
			}
			spawner.template.kwargs.insert("home".to_string(), Some(position.pos).to_parameter());
			if triggerbox.has_message(&[Trigger::Spawn]) {
				if *clan_nums.get(&spawner.clan).unwrap_or(&0) < spawner.amount {
					if spawner.saturated {
						spawner.saturated = false;
					} else {
						match new.encyclopedia.construct(&spawner.template) {
							Ok(mut preent) => {
								preent.push(ComponentWrapper::Clan(spawner.clan.clone()));
								preent.push(ComponentWrapper::Home(Home{home: position.pos}));
								let offset = Pos::new(
									rng.gen::<i64>()%(spawner.radius*2+1)-spawner.radius,
									rng.gen::<i64>()%(spawner.radius*2+1)-spawner.radius);
								new.to_build.push((position.pos + offset, preent));
							}
							Err(err) => {println!("Error: can not spawn entity from spawner: {}", err);}
						}
					}
				} else {
					spawner.saturated = true;
					timeoffsets.remove(entity);
				}
			}
		}
	}
}
