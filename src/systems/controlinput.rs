
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

use crate::pos::Pos;

use crate::components::{
	Controller,
	Played
};

use crate::controls::{
	Control,
	Action
};

use crate::resources::{
	Input,
	NewEntities
};


use crate::assemblages::Player;


pub struct ControlInput;
impl <'a> System<'a> for ControlInput {
	type SystemData = (Entities<'a>, Read<'a, Input>, WriteStorage<'a, Controller>, ReadStorage<'a, Played>, Write<'a, NewEntities>);
	fn run(&mut self, (entities, input, mut controllers, players, mut new): Self::SystemData) {
		let mut playercontrols: HashMap<&str, Control> = HashMap::new();
		let mut leaving = HashSet::new();
		for action in &input.actions {
			match action {
				Action::Join(name) => {new.assemblages.push((Pos{x:10, y:10}, Box::new(Player::new(&name))));}
				Action::Leave(name) => {leaving.insert(name);}
				Action::Input(name, control) => {playercontrols.insert(name, control.clone());}
			}
		}
		for (player, entity) in (&players, &entities).join() {
			if let Some(control) = playercontrols.get(player.name.as_str()){
				let _ = controllers.insert(entity, Controller(control.clone()));
			}
			if leaving.contains(&player.name) {
				let _ = entities.delete(entity);
			}
		}
	}
}
