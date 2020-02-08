
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
	makefloor::MakeFloor,
	controlinput::ControlInput,
	view::View,
	remove::Remove,
	create::Create
};
use crate::encyclopedia::Encyclopedia;
use crate::roomtemplate::RoomTemplate;
use crate::template::Template;



pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>,
	encyclopedia: Encyclopedia
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(encyclopedia: Encyclopedia) -> Room<'a, 'b> {
		let mut world = World::new();
		
		let mut dispatcher = DispatcherBuilder::new()
			.with(ControlInput, "controlinput", &[])
			.with(MakeFloor::default(), "makefloor", &[])
			.with(Move, "move", &["makefloor", "controlinput"])
			.with(View::default(), "view", &["move"])
			.with(Create, "create", &["view", "controlinput"])
			.with(Remove, "remove", &["view", "move"])
			.build();
		
		dispatcher.setup(&mut world);
		
		
		Room {
			world,
			dispatcher,
			encyclopedia
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
				if let Err(msg) = self.add_entity(Pos{x, y}, template) {
					println!("{}", msg);
				}
			}
		}
	}
	
	pub fn add_entity(&mut self, pos: Pos, template: &Template) -> Result<(), &'static str>{
		let pre_entity = self.encyclopedia.construct(template)?;
		self.world.fetch_mut::<NewEntities>().ents.push((pos, pre_entity));
		Ok(())
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



