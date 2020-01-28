
use specs::{
	VecStorage,
	Component
};

use super::controls::Control;


#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
	pub x: i32,
	pub y: i32
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
