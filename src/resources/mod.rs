
use std::collections::{HashMap, HashSet};
use specs::{Entity, ReadStorage, Component};

use crate::{
	pos::Pos,
	controls::Control,
	worldmessages::WorldMessage,
	componentwrapper::PreEntity,
	encyclopedia::Encyclopedia,
	PlayerId,
	RoomId,
	util::Result,
	template::Template,
	components::Visible
};


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
impl Ground {
	pub fn components_on<'a, C: Component>(&self, pos: Pos, component_type: &'a ReadStorage<C>) -> Vec<&'a C> {
		self.cells.get(&pos).unwrap_or(&HashSet::new()).iter().filter_map(|e| component_type.get(*e)).collect()
	}
	
	pub fn by_height(&self, pos: &Pos, visibles: &ReadStorage<Visible>, ignore: &Entity) -> Vec<Entity> {
		let mut entities: Vec<Entity> = self.cells
			.get(&pos).unwrap_or(&HashSet::new())
			.iter()
			.cloned()
			.filter(|e| e != ignore && visibles.contains(*e))
			.collect();
		entities.sort_by(|a, b|
			visibles.get(*b).unwrap().height.partial_cmp(&visibles.get(*a).unwrap().height).unwrap()
		);
		entities
	}
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

#[derive(Default)]
pub struct Emigration {
	pub emigrants: Vec<(PlayerId, RoomId)>
}

