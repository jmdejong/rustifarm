
use std::collections::HashMap;

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
	componentwrapper::ComponentWrapper
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
		for (entity, spawner, position, triggerbox) in (&entities, &mut spawners, &positions, &triggerboxes).join() {
			if triggerbox.has_message(&[Trigger::Spawn]) {
				if *clan_nums.get(&spawner.clan).unwrap_or(&0) < spawner.amount {
					if spawner.saturated {
						spawner.saturated = false;
					} else {
						match new.encyclopedia.construct(&spawner.template) {
							Ok(mut preent) => {
								preent.push(ComponentWrapper::Clan(spawner.clan.clone()));
								preent.push(ComponentWrapper::Home(Home{home: position.pos}));
								new.to_build.push((position.pos, preent));
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
