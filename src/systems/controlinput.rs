
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

use crate::{PlayerId, hashmap};
use crate::components::{Controller, Player, Removed};
use crate::controls::{Control, Action};
use crate::resources::{Input, NewEntities, Spawn};
use crate::template::Template;
use crate::parameter::Parameter;


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
	
		let mut playercontrols: HashMap<&PlayerId, Control> = HashMap::new();
		let mut leaving = HashSet::new();
		for action in &input.actions {
			match action {
				Action::Join(player) => {
					new.templates.push((
						spawn.pos,
						Template::new("player", hashmap!("name".to_string() => Parameter::String(player.name.clone()))).unsaved()
					));
				}
				Action::Leave(player) => {leaving.insert(player);}
				Action::Input(player, control) => {playercontrols.insert(player, control.clone());}
			}
		}
		for (player, entity) in (&players, &entities).join() {
			if let Some(control) = playercontrols.get(&player.id){
				let _ = controllers.insert(entity, Controller(control.clone()));
			}
			if leaving.contains(&player.id) {
				let _ = removed.insert(entity, Removed);
			}
		}
	}
}

