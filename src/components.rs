
use specs::{
	DenseVecStorage,
	Component
};

use super::controls::Control;

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
