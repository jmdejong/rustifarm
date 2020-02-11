
use specs::{
	DenseVecStorage,
	VecStorage,
	HashMapStorage,
	FlaggedStorage,
	Component
};

use super::controls::Control;
use super::pos::Pos;
use crate::template::Template;


#[derive(Debug, Clone)]
pub struct Position{
	pub pos: Pos
}
impl Position {
	pub fn new(pos: Pos) -> Position {
		Position{pos}
	}
}

impl Component for Position {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug, Clone)]
pub struct Visible {
	pub sprite: String,
	pub height: f64
}
impl Component for Visible {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Component, Debug)]
pub struct Controller(pub Control);

#[derive(Component, Debug, Clone)]
pub struct Blocking;

#[derive(Component, Debug, Clone)]
pub struct Floor;

#[derive(Component, Debug, Clone)]
pub struct New;

#[derive(Component, Debug, Clone)]
pub struct Removed;

#[derive(Component, Debug, Clone)]
pub struct Moved {
	pub from: Pos
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Player {
	pub name: String
}
impl Player {
	pub fn new(name: String) -> Self {
		Self{name}
	}
}

#[derive(Debug, Clone, Default)]
pub struct Inventory {
	pub items: Vec<Item>,
	pub capacity: usize
}
impl Component for Inventory {
	type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}


#[derive(Component, Debug, Clone)]
pub struct Item {
	pub ent: Template,
	pub name: String
}

#[derive(Component, Debug, Clone)]
pub struct Health {
	pub health: i64,
	pub maxhealth: i64
}

