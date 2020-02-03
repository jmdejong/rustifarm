
use specs::{
	ReadStorage,
	Write,
	System,
	Join
};

use super::super::components::{
	Position,
	Visible
};

use super::super::resources::{
	TopView
};


pub struct Draw;
impl <'a> System<'a> for Draw {
	
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Write<'a, TopView>);
	
	fn run(&mut self, (pos, vis, mut view): Self::SystemData) {
		view.cells.clear();
		for (pos, vis) in (&pos, &vis).join(){
			view.cells.entry(*pos).or_insert(Vec::new()).push(vis.clone());
			view.cells.get_mut(pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
		}
	}
}
