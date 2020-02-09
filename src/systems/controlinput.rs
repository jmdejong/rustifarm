
use std::collections::{HashMap, HashSet};

use specs::{
	ReadStorage,
	WriteStorage,
	Read,
	Write,
	Entities,
	System,
	Join
};

use crate::components::{Controller, Player, Removed};
use crate::controls::{Control, Action};
use crate::resources::{Input, NewEntities, Spawn};
use crate::hashmap;
use crate::template::Template;
use crate::parameter::Parameter;

// use crate::assemblages::Player;


pub struct ControlInput;
impl <'a> System<'a> for ControlInput {
	type SystemData = (
		Entities<'a>,
		Read<'a, Input>,
		WriteStorage<'a, Controller>,
		ReadStorage<'a, Player>,
		Write<'a, NewEntities>,
		Read<'a, Spawn>,
		WriteStorage<'a, Removed>
	);
	fn run(&mut self, (entities, input, mut controllers, players, mut new, spawn, mut removed): Self::SystemData) {
		{
			let mut ents = Vec::new();
			for (ent, _controller) in (&*entities, &controllers).join() {
				ents.push(ent);
			}
			for ent in ents {
				controllers.remove(ent);
			}
		}
	
		let mut playercontrols: HashMap<&str, Control> = HashMap::new();
		let mut leaving = HashSet::new();
		for action in &input.actions {
			match action {
				Action::Join(name) => {
					new.templates.push((
						spawn.pos,
						Template::new("player", hashmap!("name".to_string() => Parameter::String(name.to_string())))
					));
				}
				Action::Leave(name) => {leaving.insert(name);}
				Action::Input(name, control) => {playercontrols.insert(name, control.clone());}
			}
		}
		for (player, entity) in (&players, &entities).join() {
			if let Some(control) = playercontrols.get(player.name.as_str()){
				let _ = controllers.insert(entity, Controller(control.clone()));
			}
			if leaving.contains(&player.name) {
				let _ = removed.insert(entity, Removed);
			}
		}
	}
}

