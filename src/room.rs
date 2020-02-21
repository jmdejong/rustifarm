
use std::collections::HashMap;

use specs::{
	World,
	WorldExt,
	DispatcherBuilder,
	Dispatcher,
	Builder,
	Join,
	Entity
};

use super::controls::Control;
use super::worldmessages::WorldMessage;
use super::resources::{
	Size,
	Output,
	Input,
	NewEntities,
	Spawn,
	Players
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
use crate::components::{
	Position,
	Serialise,
	Player,
	Inventory,
	Health,
	New,
	Removed
};
use crate::encyclopedia::Encyclopedia;
use crate::roomtemplate::RoomTemplate;
use crate::savestate::SaveState;
use crate::template::Template;
use crate::playerstate::PlayerState;
use crate::{Pos, PlayerId, RoomId, aerr};
use crate::util::Result;



pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>,
	pub id: RoomId
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(id: RoomId, encyclopedia: Encyclopedia) -> Room<'a, 'b> {
		let mut world = World::new();
		world.insert(NewEntities::new(encyclopedia));
		world.insert(Players::default());
		world.insert(Spawn::default());
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
			dispatcher,
			id
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
				let _ = self.create_entity(template.clone().unsaved(), Pos{x, y});
			}
		}
	}
	
	
	pub fn create(id: RoomId, encyclopedia: &Encyclopedia, template: &RoomTemplate) -> Room<'a, 'b> {
		let mut room = Self::new(id, encyclopedia.clone());
		room.load_from_template(template);
		room
	}
	
	pub fn view(&self) -> HashMap<PlayerId, WorldMessage> {
		self.world.fetch::<Output>().output.clone()
	}
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&self.world);
		self.world.maintain();
	}
	
	
	pub fn control_player(&mut self, player: PlayerId, control: Control){
		self.world.fetch_mut::<Input>().actions.insert(player, control);
	}
	
	pub fn add_player(&mut self, state: &PlayerState){
		let pre_player = state.construct();
		let spawn = self.world.fetch::<Spawn>().pos;
		let mut builder = self.world.create_entity();
		let ent = builder.entity;
		for comp in pre_player {
			builder = comp.build(builder);
		}
		builder.with(Position::new(spawn)).with(New).build();
		self.world.fetch_mut::<Players>().entities.insert(state.id.clone(), ent);
	}
	
	pub fn remove_player(&mut self, id: PlayerId) -> Result<PlayerState>{
		let ent = self.world.fetch_mut::<Players>().entities.remove(&id).ok_or(aerr!("failed to remove player"))?;
		self.world.write_component::<Removed>().insert(ent, Removed)?;
		self.save_player_ent(ent).ok_or(aerr!("failed to find player to remove"))
	}
	
	pub fn save(&self) -> SaveState {
		let positions = self.world.read_component::<Position>();
		let serialisers = self.world.read_component::<Serialise>();
		let mut state = SaveState::new();
		for (pos, serialiser) in (&positions, &serialisers).join() {
			state.changes.entry(pos.pos).or_insert_with(Vec::new).push(serialiser.template.clone());
		}
		state
	}
	
	pub fn load_saved(&mut self, state: &SaveState) {
		for (pos, templates) in state.changes.iter() {
			for template in templates {
				let _ = self.create_entity(template.clone(), *pos);
			}
		}
	}
	
	pub fn save_players(&self) -> HashMap<PlayerId, PlayerState> {
		let players = self.world.read_component::<Player>();
		let inventories = self.world.read_component::<Inventory>();
		let healths = self.world.read_component::<Health>();
		let mut saved = HashMap::new();
		for (player, inventory, health) in (&players, &inventories, &healths).join() {
			saved.insert(player.id.clone(), PlayerState::create(
				player.id.clone(),
				self.id.clone(),
				inventory.items.iter().map(|item| item.ent.clone()).collect(),
				inventory.capacity,
				health.health,
				health.maxhealth
			));
		}
		saved
	}
	
	fn save_player_ent(&self, ent: Entity) -> Option<PlayerState> {
		let players = self.world.read_component::<Player>();
		let player = players.get(ent)?;
		let inventories = self.world.read_component::<Inventory>();
		let inventory = inventories.get(ent)?;
		let healths = self.world.read_component::<Health>();
		let health = healths.get(ent)?;
		Some(PlayerState::create(
			player.id.clone(),
			self.id.clone(),
			inventory.items.iter().map(|item| item.ent.clone()).collect(),
			inventory.capacity,
			health.health,
			health.maxhealth
		))
	}
	
	fn create_entity(&mut self, template: Template, pos: Pos) -> Result<()>{
		self.world.fetch_mut::<NewEntities>().create(pos, template)?;
		Ok(())
	}
}



