
use std::collections::HashMap;
use specs::Entity;

use super::components::{Position, Visible};
use super::controls::Action;
use super::assemblages::Assemblage;
use super::worldmessages::WorldMessage;


#[derive(Default)]
pub struct Input {
	pub actions: Vec<Action>
}

#[derive(Default)]
pub struct Output {
	pub output: HashMap<String, WorldMessage>
}

#[derive(Default)]
pub struct Size {
	pub width: i32,
	pub height: i32
}

#[derive(Default)]
pub struct TopView {
	pub cells: HashMap<Position, Vec<Visible>>
}


#[derive(Default)]
pub struct Floor {
	pub cells: HashMap<Position, Vec<Entity>>
}

#[derive(Default)]
pub struct NewEntities {
	pub assemblages: Vec<(Position, Box<dyn Assemblage>)>
}
