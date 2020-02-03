
use std::collections::HashMap;

use specs::{
	ReadStorage,
	Read,
	Write,
	System,
	Join
};

use super::super::pos::Pos;

use super::super::components::{
	Visible,
	Played
};

use super::super::resources::{
	Size,
	Output
};

use super::super::worldmessages::{
	WorldMessage,
	WorldUpdate,
	FieldMessage
};



pub struct View;
impl <'a> System<'a> for View {
	type SystemData = (ReadStorage<'a, Pos>, ReadStorage<'a, Visible>, Read<'a, Size>, ReadStorage<'a, Played>, Write<'a, Output>);
	fn run(&mut self, (positions, visible, size, players, mut output): Self::SystemData) {
		
		
		let mut cells: HashMap<Pos, Vec<Visible>> = HashMap::new();
		for (pos, vis) in (&positions, &visible).join(){
			cells.entry(*pos).or_insert(Vec::new()).push(vis.clone());
			cells.get_mut(pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
		}
		let width = size.width;
		let height = size.height;
		let (values, mapping) = draw_room(cells, (width, height));
		
		let field = WorldUpdate::Field(FieldMessage{
			width,
			height,
			field: values,
			mapping
		});
		output.output.clear();
		for (player, pos) in (&players, &positions).join() {
			
			let message = WorldMessage{updates: vec![
				field.clone(),
				WorldUpdate::Pos(*pos)
			]};
			output.output.insert(player.name.clone(), message);
		}
	}
}


fn draw_room(cells: HashMap<Pos, Vec<Visible>>, (width, height): (i32, i32)) -> (Vec<usize>, Vec<Vec<String>>){
	
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<String>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<String> = match cells.get(&Pos{x: x, y: y}) {
				Some(sprites) => {sprites.iter().map(|v| v.sprite.clone()).collect()}
				None => {vec![]}
			};
			values.push(
				match mapping.iter().position(|x| x == &sprites) {
					Some(index) => {
						index
					}
					None => {
						mapping.push(sprites);
						mapping.len() - 1
					}
				}
			)
		}
	}
	(values, mapping)
}
