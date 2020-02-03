
use std::collections::{HashMap, HashSet};

use specs::{
	BitSet,
	storage::ComponentEvent,
	ReaderId,
	World,
	SystemData,
	ReadStorage,
	WriteStorage,
	Read,
	Write,
	System,
	Join
};

use super::super::pos::Pos;
use super::super::components::{Visible, Played, Position};
use super::super::resources::{Size, Output};
use super::super::worldmessages::{WorldMessage, WorldUpdate, FieldMessage};


#[derive(Default)]
pub struct View {
    reader_id: Option<ReaderId<ComponentEvent>>,
    dirty: BitSet
}

impl <'a> System<'a> for View {
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Read<'a, Size>, WriteStorage<'a, Played>, Write<'a, Output>);
	fn run(&mut self, (positions, visible, size, mut players, mut output): Self::SystemData) {
		
		let mut cells: HashMap<Pos, Vec<Visible>> = HashMap::new();
		for (pos, vis) in (&positions, &visible).join(){
			cells.entry(pos.pos).or_insert(Vec::new()).push(vis.clone());
			cells.get_mut(&pos.pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
		}
		let width = size.width;
		let height = size.height;
		let (values, mapping) = draw_room(cells.clone(), (width, height));
		
		let field = WorldUpdate::Field(FieldMessage{
			width,
			height,
			field: values,
			mapping
		});
		
		
		self.dirty.clear();
		{
			let events = positions.channel().read(self.reader_id.as_mut().unwrap());
			for event in events {
				match event {
					ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) | ComponentEvent::Removed(id) => { 
						self.dirty.add(*id);
					}
				};
			}
		}
		let mut changed: HashSet<Pos> = HashSet::new();
		for (pos, _) in (&positions, &self.dirty).join(){
			changed.insert(pos.pos);
			if let Some(prev) = pos.prev{
				changed.insert(prev);
			}
		}
		let has_changed: bool = changed.len() > 0;
		let mut changes: Vec<(Pos, Vec<String>)> = Vec::new();
		for pos in changed {
			changes.push((pos, cells.get(&pos).unwrap().iter().map(|v| v.sprite.clone()).collect()));
		}
		let changed_msg = WorldUpdate::Change(changes);
		
		
		output.output.clear();
		for (mut player, pos) in (&mut players, &positions).join() {
			let mut updates: Vec<WorldUpdate> = Vec::new();
			if player.is_new {
				updates.push(field.clone());
			} else if has_changed {
				updates.push(changed_msg.clone());
			}
			updates.push(WorldUpdate::Pos(pos.pos));
			let message = WorldMessage{updates};
			output.output.insert(player.name.clone(), message);
			player.is_new = false;
		}
	}
	
	fn setup(&mut self, world: &mut World) {
		Self::SystemData::setup(world);
		self.reader_id = Some(
			WriteStorage::<Position>::fetch(&world).register_reader()
		);
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
