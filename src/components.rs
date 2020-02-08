
use specs::{
	DenseVecStorage,
	VecStorage,
	HashMapStorage,
	FlaggedStorage,
	Component
};

use super::controls::Control;
use super::pos::Pos;


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

