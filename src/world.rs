
use std::collections::HashMap;

use crate::{
	PlayerId,
	RoomId,
	room::Room,
	worldloader::WorldLoader,
	persistence::PersistentStorage,
	playerstate::PlayerState,
	encyclopedia::Encyclopedia,
	controls::Control,
	util::Result,
	aerr,
	worldmessages::WorldMessage
};

pub struct World<'a, 'b> {
	template_loader: WorldLoader,
	persistence: Box<dyn PersistentStorage>,
	default_room: RoomId,
	players: HashMap<PlayerId, RoomId>,
	rooms: HashMap<RoomId, Room<'a, 'b>>,
	encyclopedia: Encyclopedia
}


impl <'a, 'b>World<'a, 'b> {
	
	pub fn new(encyclopedia: Encyclopedia, template_loader: WorldLoader, persistence: Box<dyn PersistentStorage>, default_room: RoomId) -> Self {
		World {
			template_loader,
			persistence,
			default_room,
			encyclopedia,
			players: HashMap::new(),
			rooms: HashMap::new()
		}
	}
	
	fn get_room_mut(&mut self, id: &RoomId) -> Option<&mut Room<'a, 'b>> {
		if !self.rooms.contains_key(id){
			let template = self.template_loader.load_room(id.clone()).ok()?;
			let mut room: Room = Room::create(id.clone(), &self.encyclopedia, &template);
			if let Ok(state) = self.persistence.load_room(id.clone()){
				room.load_saved(&state);
			}
			self.rooms.insert(id.clone(), room);
		}
		self.rooms.get_mut(id)
	}
	
	fn add_loaded_player(&mut self, state: PlayerState) -> Result<()> {
		let roomid = state.clone().room.unwrap_or(self.default_room.clone());
		let room = self.get_room_mut(&roomid).ok_or(aerr!("room not found"))?;
		room.add_player(&state);
		self.players.insert(state.id.clone(), roomid);
		Ok(())
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let state = self.persistence.load_player(playerid.clone()).unwrap_or(PlayerState::new(playerid.clone()));
		self.add_loaded_player(state)
	}
	
	fn discorporate_player(&mut self, playerid: &PlayerId) -> Result<PlayerState> {
		let roomid = self.players.remove(playerid).ok_or(aerr!("player not found"))?;
		let room = self.get_room_mut(&roomid).ok_or(aerr!("room not found"))?;
		room.remove_player(playerid)
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let player_state = self.discorporate_player(playerid)?;
		self.persistence.save_player(playerid.clone(), player_state)?;
		Ok(())
	}
	
	
	pub fn control_player(&mut self, player: PlayerId, control: Control) -> Result<()>{
		let roomid = self.players.get(&player).ok_or(aerr!("player not found"))?.clone();
		Ok(self.get_room_mut(&roomid).ok_or(aerr!("room not found"))?.control_player(player, control))
	}
	
	fn migrate_player(&mut self, player: &PlayerId, destination: RoomId) -> Result<()> {
		let mut state = self.discorporate_player(player)?;
		state.room = Some(destination);
		self.add_loaded_player(state)
	}
		
	
	pub fn update(&mut self) {
		let mut migrants = Vec::new();
		for room in self.rooms.values_mut() {
			room.update();
			migrants.append(&mut room.emigrate());
		}
		for (player, destination) in migrants {
			self.migrate_player(&player, destination).unwrap();
		}
	}
	
	pub fn save(&self) {
		for room in self.rooms.values() {
			if let Err(err) = self.persistence.save_room(room.id.clone(), room.save()) {
				println!("{:?}",err);
			} else {
				println!("{}", room.save().to_json());
			}
			for (playerid, state) in room.save_players() {
				if let Err(err) = self.persistence.save_player(playerid.clone(), state.clone()) {
					println!("{:?}",err);
				} else {
					println!("{:?} {}", playerid, state.to_json());
				}
			}
		}
	}
	
	pub fn view(&self) -> HashMap<PlayerId, WorldMessage> {
		let mut views = HashMap::new();
		for room in self.rooms.values() {
			for (player, message) in room.view().into_iter() {
				views.insert(player, message);
			}
		}
		views
	}
}
