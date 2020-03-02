
use specs::{
	ReadStorage,
	WriteStorage,
	Write,
	Entities,
	System,
	Join
};

use crate::{
	components::{Controller, Player, ControlCooldown, Autofight},
	resources::{Input},
	controls::Control
};


pub struct ControlInput;
impl <'a> System<'a> for ControlInput {
	type SystemData = (
		Entities<'a>,
		Write<'a, Input>,
		WriteStorage<'a, Controller>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, ControlCooldown>,
		WriteStorage<'a, Autofight>
	);
	fn run(&mut self, (entities, mut input, mut controllers, players, cooldowns, mut autofighters): Self::SystemData) {
	
		for (player, entity, ()) in (&players, &entities, !&cooldowns).join() {
			if let Some(control) = input.actions.remove(&player.id){
				controllers.insert(entity, Controller{control: control}).unwrap();
				if let Some(autofighter) = autofighters.get_mut(entity) {
					autofighter.target = None;
				}
			} else if let Some(autofighter) = autofighters.get_mut(entity) {
				if let Some(target) = autofighter.target {
					if !entities.is_alive(target) {
						autofighter.target = None;
					} else {
						controllers.insert(entity, Controller{control: Control::AttackTarget(target)}).unwrap();
					}
				}
			}
		}
	}
}

