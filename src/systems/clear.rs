
use specs::{
	Write,
	System
};

use crate::{
	resources::Ground
};

pub struct Clear;
impl <'a> System<'a> for Clear {
	type SystemData = 
		Write<'a, Ground>;
	fn run(&mut self, mut ground: Self::SystemData) {
		ground.changes.clear();
	}
}

