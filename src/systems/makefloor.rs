

use specs::{
	ReadStorage,
	Write,
	Entities,
	System,
	Join
};

use super::super::components::Position;

use super::super::resources::{
	Ground
};


pub struct MakeFloor;
impl <'a> System<'a> for MakeFloor {
	type SystemData = (Entities<'a>, Write<'a, Ground>, ReadStorage<'a, Position>);
	fn run(&mut self, (entities, mut ground, positions): Self::SystemData) {
		ground.cells.clear();
		for (ent, pos) in (&entities, &positions).join() {
			ground.cells.entry(pos.pos).or_insert(Vec::new()).push(ent);
		}
	}
}
