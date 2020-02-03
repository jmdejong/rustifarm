

use specs::{
	ReadStorage,
	Write,
	Entities,
	System,
	Join
};

use super::super::components::{
	Pos
};

use super::super::resources::{
	Floor
};


pub struct MakeFloor;
impl <'a> System<'a> for MakeFloor {
	type SystemData = (Entities<'a>, Write<'a, Floor>, ReadStorage<'a, Pos>);
	fn run(&mut self, (entities, mut floor, positions): Self::SystemData) {
		floor.cells.clear();
		for (ent, pos) in (&entities, &positions).join() {
			floor.cells.entry(*pos).or_insert(Vec::new()).push(ent);
		}
	}
}
