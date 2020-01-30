
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

use super::components::{
	Position,
	Visible,
	Controller,
	Blocking,
	Played
};

use super::controls::{
	Control,
	Action
};

use super::resources::{
	TopView,
	Size,
	Floor,
	Input,
	NewEntities
};

use super::assemblages::Player;


pub struct MakeFloor;
impl <'a> System<'a> for MakeFloor {
	type SystemData = (Entities<'a>, Write<'a, Floor>, ReadStorage<'a, Position>);
	fn run(&mut self, (entities, mut floor, positions): Self::SystemData) {
		floor.cells.clear();
		for (ent, pos) in (&entities, &positions).join() {
			floor.cells.entry(*pos).or_insert(Vec::new()).push(ent);
		}
	}
}

pub struct Draw;
impl <'a> System<'a> for Draw {
	
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Write<'a, TopView>);
	
	fn run(&mut self, (pos, vis, mut view): Self::SystemData) {
		view.cells.clear();
		for (pos, vis) in (&pos, &vis).join(){
			view.cells.entry(*pos).or_insert(Vec::new()).push(vis.clone());
			view.cells.get_mut(pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
		}
	}
}

pub struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (ReadStorage<'a, Controller>, WriteStorage<'a, Position>, Read<'a, Size>, ReadStorage<'a, Blocking>, Read<'a, Floor>);
	fn run(&mut self, (controller, mut pos, size, blocking, floor): Self::SystemData) {
		for (controller, pos) in (&controller, &mut pos).join(){
			match &controller.0 {
				Control::Move(direction) => {
					let newpos = (*pos + direction.to_position()).clamp(Position::new(0, 0), Position::new(size.width - 1, size.height - 1));
					let mut blocked = false;
					for ent in floor.cells.get(&newpos).unwrap_or(&Vec::new()) {
						if blocking.get(*ent).is_some(){
							blocked = true;
							break;
						}
					}
					if !blocked {
						pos.clone_from(&newpos);
					}
				}
				_ => {}
			}
		}
	}
}


pub struct ControlInput;
impl <'a> System<'a> for ControlInput {
	type SystemData = (Entities<'a>, Read<'a, Input>, WriteStorage<'a, Controller>, ReadStorage<'a, Played>, Write<'a, NewEntities>);
	fn run(&mut self, (entities, input, mut controllers, players, mut new): Self::SystemData) {
		let mut playercontrols: HashMap<&str, Control> = HashMap::new();
		let mut leaving = HashSet::new();
		for action in &input.actions {
			match action {
				Action::Join(name) => {new.assemblages.push((Position{x:10, y:10}, Box::new(Player::new(&name))));}
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


pub struct ClearControllers;
impl <'a> System<'a> for ClearControllers {
	type SystemData = (Entities<'a>, WriteStorage<'a, Controller>);
	fn run(&mut self, (entities, mut controllers): Self::SystemData) {
		let mut ents = Vec::new();
		for (ent, _controller) in (&*entities, &controllers).join() {
			ents.push(ent);
		}
		for ent in ents {
			controllers.remove(ent);
		}
	}
}

