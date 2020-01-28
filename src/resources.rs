
use std::collections::HashMap;

use super::components::{Position, Visible};

#[derive(Default)]
pub struct Size (pub i32, pub i32);

#[derive(Default)]
pub struct TopView {
	pub width: i32,
	pub height: i32,
	pub cells: HashMap<Position, Vec<Visible>>
}
