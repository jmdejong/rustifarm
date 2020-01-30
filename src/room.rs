

use specs::{
	World,
	WorldExt,
	Builder,
	DispatcherBuilder,
	Dispatcher,
	Entity
};

use super::controls::Action;
use super::components::Position;
use super::assemblages::Assemblage;
use super::resources::{
	Size,
	TopView,
	Input,
	NewEntities
};
use super::systems::{
	Draw,
	Move,
	ClearControllers,
	MakeFloor,
	ControlInput
};



pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(size: (i32, i32)) -> Room<'a, 'b> {
		let (width, height) = size;
		let mut world = World::new();
		world.insert(Size{width, height});
		world.insert(Input{actions: Vec::new()});
		
		let mut dispatcher = DispatcherBuilder::new()
			.with(ControlInput, "controlinput", &[])
			.with(MakeFloor, "makefloor", &[])
			.with(Move, "move", &["makefloor", "controlinput"])
			.with(Draw, "draw", &["move"])
			.with(ClearControllers, "clearcontrollers", &["move"])
			.build();
		
		dispatcher.setup(&mut world);
		
		Room {
			world,
			dispatcher
		}
	}
	
	pub fn view(&self) -> (Vec<usize>, Vec<Vec<String>>) {
		let tv = &*self.world.fetch::<TopView>();
		let (width, height) = self.get_size();
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
		let assemblages = self.world.remove::<NewEntities>().unwrap_or(NewEntities{assemblages: Vec::new()}).assemblages;
		self.world.insert(NewEntities{assemblages: Vec::new()});
		for (pos, assemblage) in assemblages{
			assemblage.build(self.world.create_entity()).with(pos).build();
		}
		self.world.maintain();
	}
	
	pub fn get_size(&self) -> (i32, i32) {
		let Size{width, height} = *self.world.fetch::<Size>();
		(width, height)
	}
	
	pub fn set_input(&mut self, actions: Vec<Action>){
		self.world.fetch_mut::<Input>().actions = actions;
	}
	
	pub fn add_obj(&mut self, template: &dyn Assemblage, (x, y): (i32, i32)) -> Entity {
		template.build(self.world.create_entity()).with(Position{x, y}).build()
	}
}



