
use std::collections::HashMap;

use specs::{
	WriteStorage,
	ReadStorage,
	Write,
	Read,
	System,
	Join
};

use crate::{
	components::{
		Position,
		Spawner,
		Clan,
		Home
	},
	resources::{NewEntities, Time},
	componentwrapper::ComponentWrapper
};



pub struct Spawn;
impl <'a> System<'a> for Spawn {
	type SystemData = (
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Spawner>,
		ReadStorage<'a, Clan>,
		Read<'a, Time>
	);
	
	fn run(&mut self, (positions, mut new, mut spawners, clans, time): Self::SystemData) {
		let mut clan_nums: HashMap<&Clan, usize> = HashMap::new();
		for clan in (&clans).join() {
			let n: usize = *clan_nums.entry(clan).or_insert(0);
			clan_nums.insert(clan, n+1);
		}
		for (spawner, position) in (&mut spawners, &positions).join() {
			if *clan_nums.get(&spawner.clan).unwrap_or(&0) < spawner.amount {
				if let Some(last_spawn) = spawner.last_spawn {
					if time.time > last_spawn + spawner.delay {
						spawner.last_spawn = None;
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
					spawner.last_spawn = Some(time.time)
				}
			}
		}
	}
}
