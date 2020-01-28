
use std::collections::HashMap;
use specs::{
	World,
	WorldExt,
	Builder,
	DispatcherBuilder,
	Dispatcher,
	Entity
};

use super::controls::Control;
use super::components::{Position, Visible, Controller};
use super::assemblages::Assemblage;
use super::resources::{Size, TopView};
use super::systems::{Draw, Move, ClearControllers};



pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>,
	spawn: (i32, i32),
	players: HashMap<String, Entity>
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(size: (i32, i32)) -> Room<'a, 'b> {
		let (width, height) = size;
		let mut world = World::new();
		world.register::<Position>();
		world.register::<Visible>();
		world.register::<Controller>();
		world.insert(Size(width, height));
		world.insert(TopView{width: width, height: height, cells: HashMap::new()});
		
		let dispatcher = DispatcherBuilder::new()
			.with(Move, "move", &[])
			.with(Draw, "draw", &["move"])
			.with(ClearControllers, "clearcontrollers", &["move"])
			.build();
		
		Room {
			world,
			dispatcher,
			spawn: (width / 2, height / 2),
			players: HashMap::new()
		}
	}
	
	pub fn view(&self) -> (Vec<usize>, Vec<Vec<String>>) {
		let tv = &*self.world.fetch::<TopView>();
		let width = tv.width;
		let height = tv.height;
		let size = width * height;
		let mut values :Vec<usize> = Vec::with_capacity(size as usize);
		let mut mapping: Vec<Vec<String>> = Vec::new();
		for y in 0..height {
			for x in 0..width {
				let sprites: Vec<String> = match tv.cells.get(&Position{x: x, y: y}) {
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
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&mut self.world);
		self.world.maintain();
	}
	
	pub fn get_size(&self) -> (i32, i32) {
		let Size(width, height) = *self.world.fetch::<Size>();
		(width, height)
	}
	
	pub fn add_obj(&mut self, template: &dyn Assemblage, (x, y): (i32, i32)) -> Entity {
		template.build(self.world.create_entity()).with(Position{x, y}).build()
	}
	
	pub fn add_player(&mut self, name: &str, template: &dyn Assemblage) {
		let ent = self.add_obj(template, self.spawn);
		self.players.insert(name.to_string(), ent);
	}
	
	pub fn remove_player(&mut self, name: &str){
		// todo: proper error handling
		let ent = self.players.remove(name).expect("unknown player name");
		self.world.delete_entity(ent).expect("player in world does not have entity");
	}
	
// 	pub fn clear_controls(&mut self){
// 		(*self.world.fetch_mut::<Controls>()).0.clear();
// 	}
	
	pub fn control(&mut self, name: String, control: Control){
		if let Some(ent) = self.players.get(&name){
			let _ = self.world.write_component::<Controller>().insert(*ent, Controller(control));
		}
	}
}



