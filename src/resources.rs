
use std::collections::{HashMap, HashSet};
use specs::Entity;

use super::pos::Pos;
use super::controls::Action;
use super::worldmessages::WorldMessage;
use super::template::Template;
use crate::encyclopedia::Encyclopedia;
use crate::PlayerId;


#[derive(Default)]
pub struct Input {
	pub actions: Vec<Action>
}

#[derive(Default)]
pub struct Output {
	pub output: HashMap<PlayerId, WorldMessage>
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
	pub templates: Vec<(Pos, Template)>,
	pub encyclopedia: Encyclopedia
}
