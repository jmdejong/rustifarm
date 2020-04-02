
use std::collections::HashSet;
use specs::{
	Component,
	VecStorage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
	Blocking,
	Floor,
	Occupied,
	Soil
}

use Flag::*;
impl Flag {
	pub fn from_str(s: &str) -> Option<Flag> {
		Some(match s {
			"Blocking" => Blocking,
			"Floor" => Floor,
			"Occupied" => Occupied,
			"Soil" => Soil,
			_ => None?
		})
	}
}


#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Flags(pub HashSet<Flag>);
