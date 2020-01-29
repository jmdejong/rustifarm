
use std::collections::HashMap;
use specs::Entity;

use super::components::{Position, Visible};

#[derive(Default)]
pub struct Size {
	pub width: i32,
	pub height: i32
}

#[derive(Default)]
pub struct TopView {
	pub cells: HashMap<Position, Vec<Visible>>
}


#[derive(Default)]
pub struct Floor {
	pub cells: HashMap<Position, Vec<Entity>>
}
