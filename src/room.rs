
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
	clearcontrols::ClearControllers,
	makefloor::MakeFloor,
	controlinput::ControlInput,
	view::View
};
use super::componentwrapper::ComponentWrapper;
use crate::encyclopedia::Encyclopedia;
use crate::template::Template;
use crate::roomtemplate::RoomTemplate;



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
			.with(MakeFloor, "makefloor", &[])
			.with(Move, "move", &["makefloor", "controlinput"])
			.with(ClearControllers, "clearcontrollers", &["move"])
			.with(View::default(), "view", &["move"])
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
				if let Err(msg) = self.add_entity(template, Pos{x, y}){
					println!("{}", msg);
				}
			}
		}
	}
	
	pub fn view(&self) -> HashMap<String, WorldMessage> {
		self.world.fetch::<Output>().output.clone()
	}
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&mut self.world);
		let templates = self.world.remove::<NewEntities>().unwrap_or(NewEntities::default()).templates;
		self.world.insert(NewEntities::default());
		for (pos, template) in templates{
			if let Err(msg) = self.add_entity(&template, pos){
				println!("failed to add entity {:?}: {}", template, msg);
			}
		}
		self.world.maintain();
	}
	
	pub fn set_input(&mut self, actions: Vec<Action>){
		self.world.fetch_mut::<Input>().actions = actions;
	}
	
	pub fn add_entity(&mut self, template: &Template, pos: Pos) -> Result<Entity, &'static str> {
		let preentity = self.encyclopedia.construct(template)?;
		Ok(self.add_complist(&preentity, pos))
	}
// 	
	pub fn add_complist(&mut self, template: &Vec<ComponentWrapper>, pos: Pos) -> Entity{
		let mut builder = self.world.create_entity();
		for comp in template {
			builder = comp.build(builder);
		}
		builder.with(Position::new(pos)).build()
	}
}



