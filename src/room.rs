
use std::collections::HashMap;

use specs::{
	World,
	WorldExt,
	DispatcherBuilder,
	Dispatcher
};

use super::controls::Action;
use super::pos::Pos;
use super::worldmessages::WorldMessage;
use super::resources::{
	Size,
	Output,
	Input,
	NewEntities,
	Spawn
};
use super::systems::{
	moving::Move,
	registernew::RegisterNew,
	controlinput::ControlInput,
	view::View,
	remove::Remove,
	create::Create
};
use crate::encyclopedia::Encyclopedia;
use crate::roomtemplate::RoomTemplate;



pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(encyclopedia: Encyclopedia) -> Room<'a, 'b> {
		let mut world = World::new();
		world.insert(NewEntities{
			templates: Vec::new(),
			encyclopedia
		});
		
		let mut dispatcher = DispatcherBuilder::new()
			.with(ControlInput, "controlinput", &[])
			.with(RegisterNew::default(), "makefloor", &[])
			.with(Move, "move", &["makefloor", "controlinput"])
			.with(View::default(), "view", &["move"])
			.with(Create, "create", &["view", "controlinput"])
			.with(Remove, "remove", &["view", "move"])
			.build();
		
		dispatcher.setup(&mut world);
		
		
		Room {
			world,
			dispatcher
		}
	}
	
	pub fn load_from_template(&mut self, template: &RoomTemplate) {
	
		let (width, height) = template.size;
		self.world.fetch_mut::<Size>().width = width;
		self.world.fetch_mut::<Size>().height = height;
		
		self.world.fetch_mut::<Spawn>().pos = template.spawn;
		
		for (idx, templates) in template.field.iter().enumerate() {
			let x = (idx as i64) % width;
			let y = (idx as i64) / width;
			
			for template in templates {
				self.world.fetch_mut::<NewEntities>().templates.push((Pos{x, y}, template.clone()));
			}
		}
	}
	
	pub fn view(&self) -> HashMap<String, WorldMessage> {
		self.world.fetch::<Output>().output.clone()
	}
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&mut self.world);
		self.world.maintain();
	}
	
	pub fn set_input(&mut self, actions: Vec<Action>){
		self.world.fetch_mut::<Input>().actions = actions;
	}
	
}



