
use specs::{
	DenseVecStorage,
	VecStorage,
	FlaggedStorage,
	Component
};

use super::controls::Control;
use super::pos::Pos;


#[derive(Debug, Clone)]
pub struct Position{
	pub pos: Pos,
	pub prev: Option<Pos>
}
impl Position {
	pub fn new(pos: Pos) -> Position {
		Position{pos, prev: None}
	}
}

impl Component for Position {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug, Clone)]
pub struct Visible {
	pub sprite: String,
	pub height: f32
}
impl Component for Visible {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Component, Debug)]
pub struct Controller(pub Control);

#[derive(Component, Debug)]
pub struct Blocking;

#[derive(Component, Debug)]
pub struct Played {
	pub name: String,
	pub is_new: bool
}
impl Played {
	pub fn new(name: String) -> Played {
		Played{name, is_new: true}
	}
}
