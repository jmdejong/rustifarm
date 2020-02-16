
use std::collections::HashMap;

use specs::{
	World,
	WorldExt,
	DispatcherBuilder,
	Dispatcher,
	Join
};

use super::controls::Action;
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
	create::Create,
	take::Take
};
use crate::components::{Position, Serialise};
use crate::encyclopedia::Encyclopedia;
use crate::roomtemplate::RoomTemplate;
use crate::savestate::SaveState;
use crate::{Pos, PlayerId};



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
		world.register::<Serialise>();
		
		let mut dispatcher = DispatcherBuilder::new()
			.with(RegisterNew::default(), "registernew", &[])
			.with(ControlInput, "controlinput", &[])
			.with(Take, "take", &["controlinput"])
			.with(Move, "move", &["registernew", "controlinput"])
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
				self.world.fetch_mut::<NewEntities>().templates.push((Pos{x, y}, template.clone().unsaved()));
			}
		}
	}
	
	pub fn view(&self) -> HashMap<PlayerId, WorldMessage> {
		self.world.fetch::<Output>().output.clone()
	}
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&self.world);
		self.world.maintain();
	}
	
	pub fn set_input(&mut self, actions: Vec<Action>){
		self.world.fetch_mut::<Input>().actions = actions;
	}
	
	pub fn save(&self) -> SaveState {
		let positions = self.world.read_component::<Position>();
		let serialisers = self.world.write_component::<Serialise>();
		let mut state = SaveState::new();
		for (pos, serialiser) in (&positions, &serialisers).join() {
			state.changes.entry(pos.pos).or_insert(Vec::new()).push(serialiser.template.clone());
		}
		state
	}
	
}



