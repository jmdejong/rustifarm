
use std::collections::HashMap;
use specs::Entity;

use super::pos::Pos;
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
pub struct Floor {
	pub cells: HashMap<Pos, Vec<Entity>>
}

#[derive(Default)]
pub struct NewEntities {
	pub assemblages: Vec<(Pos, Box<dyn Assemblage>)>
}
