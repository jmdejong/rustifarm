
use std::ops;

use specs::{
	VecStorage,
	Component
};

use super::controls::Control;
use super::util::clamp;


#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
	pub x: i32,
	pub y: i32
}

impl ops::Add<Position> for Position {
	type Output = Position;

	fn add(self, other: Position) -> Position {
		Position {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Position {
	
	pub fn new(x: i32, y: i32) -> Position {
		Position {x, y}
	}
	
	pub fn clamp(self, smaller: Position, larger: Position) -> Position {
		Position {
			x: clamp(self.x, smaller.x, larger.x),
			y: clamp(self.y, smaller.y, larger.y)
		}
	}
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Visible {
    pub sprite: String,
    pub height: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Controller(pub Control);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Blocking;
