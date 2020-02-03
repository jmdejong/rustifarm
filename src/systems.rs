
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
	Output,
	NewEntities
};

use super::worldmessages::{
	WorldMessage,
	WorldUpdate,
	FieldMessage
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

pub struct View;
impl <'a> System<'a> for View {
	type SystemData = (Read<'a, TopView>, Read<'a, Size>, ReadStorage<'a, Played>, Write<'a, Output>);
	fn run(&mut self, (topview, size, players, mut output): Self::SystemData) {
		
		
		let width = size.width;
		let height = size.height;
		let (values, mapping) = draw_room(&topview.cells, (width, height));
		
		let message = WorldMessage{updates: vec![WorldUpdate::Field(FieldMessage{
			width,
			height,
			field: values,
			mapping
		})]};
		output.output.clear();
		for player in (&players).join() {
			output.output.insert(player.name.clone(), message.clone());
		}
	}
}

fn draw_room(cells: &HashMap<Position, Vec<Visible>>, (width, height): (i32, i32)) -> (Vec<usize>, Vec<Vec<String>>){
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<String>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<String> = match cells.get(&Position{x: x, y: y}) {
				Some(sprites) => {sprites.iter().map(|v| v.sprite.clone()).collect()}
				None => {vec![]}
			};
			values.push(
				match mapping.iter().position(|x| x == &sprites) {
					Some(index) => {
						index
					}
					None => {
						mapping.push(sprites);
						mapping.len() - 1
					}
				}
			)
		}
	}
	(values, mapping)
}

