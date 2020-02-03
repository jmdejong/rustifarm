
use std::collections::HashMap;

use specs::{
	ReadStorage,
	Read,
	Write,
	System,
	Join
};

use super::super::components::{
	Position,
	Visible,
	Played
};


use super::super::resources::{
	TopView,
	Size,
	Output,
};

use super::super::worldmessages::{
	WorldMessage,
	WorldUpdate,
	FieldMessage
};



pub struct View;
impl <'a> System<'a> for View {
	type SystemData = (Read<'a, TopView>, Read<'a, Size>, ReadStorage<'a, Played>, Write<'a, Output>);
	fn run(&mut self, (topview, size, players, mut output): Self::SystemData) {
		
		
		let width = size.width;
		let height = size.height;
		let (values, mapping) = draw_room(&topview.cells, (width, height));
		
		let message = WorldMessage{updates: vec![WorldUpdate::Field(FieldMessage{
			width,
			height,
			field: values,
			mapping
		})]};
		output.output.clear();
		for player in (&players).join() {
			output.output.insert(player.name.clone(), message.clone());
		}
	}
}



fn draw_room(cells: &HashMap<Position, Vec<Visible>>, (width, height): (i32, i32)) -> (Vec<usize>, Vec<Vec<String>>){
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<String>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<String> = match cells.get(&Position{x: x, y: y}) {
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
