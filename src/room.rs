
use std::collections::HashMap;

use specs::{
	World,
	WorldExt,
	Builder,
	DispatcherBuilder,
	Dispatcher,
	Entity
};

use super::controls::Action;
use super::pos::Pos;
use super::components::Position;
use super::oldassemblage::Assemblage as OldAssemblage;
use super::worldmessages::WorldMessage;
use super::resources::{
	Size,
	Output,
	Input,
	NewEntities
};
use super::systems::{
	moving::Move,
	clearcontrols::ClearControllers,
	makefloor::MakeFloor,
	controlinput::ControlInput,
	view::View
};
use super::componentwrapper::ComponentWrapper;



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
		world.insert(Output{output: HashMap::new()});
		
		let mut dispatcher = DispatcherBuilder::new()
			.with(ControlInput, "controlinput", &[])
			.with(MakeFloor, "makefloor", &[])
			.with(Move, "move", &["makefloor", "controlinput"])
			.with(ClearControllers, "clearcontrollers", &["move"])
			.with(View::default(), "view", &["move"])
			.build();
		
		dispatcher.setup(&mut world);
		
		Room {
			world,
			dispatcher
		}
	}
	
	pub fn view(&self) -> HashMap<String, WorldMessage> {
		self.world.fetch::<Output>().output.clone()
	}
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&mut self.world);
		let assemblages = self.world.remove::<NewEntities>().unwrap_or(NewEntities{assemblages: Vec::new()}).assemblages;
		self.world.insert(NewEntities{assemblages: Vec::new()});
		for (pos, assemblage) in assemblages{
			assemblage.build(self.world.create_entity()).with(Position::new(pos)).build();
		}
		self.world.maintain();
	}
	
	pub fn set_input(&mut self, actions: Vec<Action>){
		self.world.fetch_mut::<Input>().actions = actions;
	}
	
	pub fn add_obj(&mut self, template: &dyn OldAssemblage, (x, y): (i32, i32)) -> Entity {
		template.build(self.world.create_entity()).with(Position::new(Pos{x, y})).build()
	}
	
	pub fn add_complist(&mut self, template: &Vec<ComponentWrapper>, (x, y): (i32, i32)) -> Entity{
		let mut builder = self.world.create_entity();
		for comp in template {
			builder = comp.build(builder);
		}
		builder.with(Position::new(Pos{x, y})).build()
	}
}



