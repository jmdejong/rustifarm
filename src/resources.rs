
use std::collections::{HashMap, HashSet};
use specs::Entity;

use super::pos::Pos;
use super::controls::Action;
use super::worldmessages::WorldMessage;
use crate::componentwrapper::PreEntity;


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
	pub width: i64,
	pub height: i64
}

#[derive(Default)]
pub struct Spawn {
	pub pos: Pos
}

#[derive(Default)]
pub struct Ground {
	pub cells: HashMap<Pos, HashSet<Entity>>
}

#[derive(Default)]
pub struct NewEntities {
	pub ents: Vec<(Pos, PreEntity)>
}
