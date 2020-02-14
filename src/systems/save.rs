
use std::collections::HashMap;

use specs::{
	Entities,
	ReadStorage,
	System,
	Join,
	Read
};

use crate::pos::Pos;

use crate::components::{
	Position,
	Serialise
};

use crate::savestate::SaveState;

const INTERVAL: i32 = 20;

pub struct Save(pub i32);
impl <'a> System<'a> for Save {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Serialise>,
	);
	
	fn run(&mut self, (entities, positions, serialisers): Self::SystemData) {
		self.0 -= 1;
		if self.0 > 0 {
			return
		}
		self.0 = INTERVAL;
		let mut state = SaveState::new();
		for (pos, serialiser) in (&positions, &serialisers).join() {
			state.changes.entry(pos.pos).or_insert(Vec::new()).push(serialiser.template.clone());
		}
		println!("save {}", state.to_json().to_string());
	}
}
