
use std::collections::HashSet;
use specs::{
	Component,
	VecStorage,
};
use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Serialize, Deserialize)]
pub enum Flag {
	Blocking,
	Floor,
	Occupied,
	Soil,
	Build,
	Hot
}


#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct Flags(pub HashSet<Flag>);
