
mod ground;
mod newentities;

pub use ground::Ground;
pub use newentities::NewEntities;

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use specs::{Entity};

use crate::{
	Pos,
	controls::Control,
	worldmessages::WorldMessage,
	PlayerId,
	RoomId,
	playerstate::RoomPos,
	Timestamp,
	components::Flag
};


#[derive(Default)]
pub struct Input {
	pub actions: HashMap<PlayerId, Control>
}

#[derive(Default)]
pub struct Output {
	pub output: HashMap<PlayerId, WorldMessage>
}

#[derive(Default)]
pub struct Size {
	pub width: i64,
	pub height: i64
}

#[derive(Default)]
pub struct Spawn {
	pub pos: Pos
}


#[derive(Default)]
pub struct Players {
	pub entities: HashMap<PlayerId, Entity>
}

#[derive(Default)]
pub struct Emigration {
	pub emigrants: Vec<(PlayerId, RoomId, RoomPos)>
}

#[derive(Default)]
pub struct Time {
	pub time: Timestamp
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RoomFlags(pub HashSet<Flag>);

