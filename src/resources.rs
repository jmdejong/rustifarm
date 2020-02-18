
use std::collections::{HashMap, HashSet};
use specs::Entity;

use super::pos::Pos;
use super::controls::Control;
use super::worldmessages::WorldMessage;
use crate::componentwrapper::PreEntity;
use crate::encyclopedia::Encyclopedia;
use crate::PlayerId;
use crate::util::Result;
use crate::template::Template;


#[derive(Default)]
pub struct Input {
	pub actions: HashMap<PlayerId, Control>
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
	pub to_build: Vec<(Pos, PreEntity)>,
	pub encyclopedia: Encyclopedia
}
impl NewEntities {
	pub fn new(encyclopedia: Encyclopedia) -> Self {
		Self{
			to_build: Vec::new(),
			encyclopedia
		}
	}
	pub fn create(&mut self, pos: Pos, template: Template) -> Result<()> {
		let components = self.encyclopedia.construct(&template)?;
		self.to_build.push((pos, components));
		Ok(())
	}
}

#[derive(Default)]
pub struct Players {
	pub entities: HashMap<PlayerId, Entity>
}
