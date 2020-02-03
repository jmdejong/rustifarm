
use std::ops;

use specs::{
	DenseVecStorage,
	Component
};

use super::controls::Control;
use super::util::clamp;


#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos {
	pub x: i32,
	pub y: i32
}

impl ops::Add<Pos> for Pos {
	type Output = Pos;

	fn add(self, other: Pos) -> Pos {
		Pos {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Pos {
	
	pub fn new(x: i32, y: i32) -> Pos {
		Pos {x, y}
	}
	
	pub fn clamp(self, smaller: Pos, larger: Pos) -> Pos {
		Pos {
			x: clamp(self.x, smaller.x, larger.x),
			y: clamp(self.y, smaller.y, larger.y)
		}
	}
}

#[derive(Component, Debug, Clone)]
pub struct Visible {
    pub sprite: String,
    pub height: f32
}

#[derive(Component, Debug)]
pub struct Controller(pub Control);

#[derive(Component, Debug)]
pub struct Blocking;

#[derive(Component, Debug)]
pub struct Played {
	pub name: String
}
