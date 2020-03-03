
use std::collections::HashMap;
use crate::hashmap;

use crate::{
	PlayerId,
	RoomId,
	room::Room,
	worldloader::WorldLoader,
	persistence::PersistentStorage,
	playerstate::{PlayerState, RoomPos},
	Encyclopedia,
	controls::Control,
	Result,
	aerr,
	worldmessages::WorldMessage,
	Timestamp,
	purgatory
};

pub struct World<'a, 'b> {
	template_loader: WorldLoader,
	persistence: Box<dyn PersistentStorage>,
	default_room: RoomId,
	players: HashMap<PlayerId, RoomId>,
	rooms: HashMap<RoomId, Room<'a, 'b>>,
	encyclopedia: Encyclopedia,
	time: Timestamp
}

impl <'a, 'b>World<'a, 'b> {
	
	pub fn new(encyclopedia: Encyclopedia, template_loader: WorldLoader, persistence: Box<dyn PersistentStorage>, default_room: RoomId) -> Self {
		World {
			template_loader,
			persistence,
			default_room,
			encyclopedia: encyclopedia.clone(),
			players: HashMap::new(),
			rooms: hashmap!(purgatory::purgatory_id() => purgatory::create_purgatory(encyclopedia)),
			time: Timestamp(1000000)
		}
	}
	
	fn get_room_mut(&mut self, id: &RoomId) -> Result<&mut Room<'a, 'b>> {
		if !self.rooms.contains_key(id){
			let template = self.template_loader.load_room(id.clone())?;
			let mut room: Room = Room::create(id.clone(), &self.encyclopedia, &template);
			if let Ok(state) = self.persistence.load_room(id.clone()){
				room.load_saved(&state);
			}
		let last_time = self.time - 1;
			if room.get_time() < last_time {
				room.update(last_time);
			}
			self.rooms.insert(id.clone(), room);
		}
		// todo: log any that occurs (and still return it)
		self.rooms.get_mut(id).ok_or(aerr!("can't get room after loading it"))
	}
	
	fn add_loaded_player(&mut self, state: PlayerState) -> Result<()> {
		let roomid = state.clone().room.unwrap_or_else(|| self.default_room.clone());
		let room = self.get_room_mut(&roomid)?;
		room.add_player(&state);
		self.players.insert(state.id, roomid);
		Ok(())
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let mut state = self.persistence
			.load_player(playerid.clone())
			.unwrap_or_else(|_err| // todo: what if player exists but can't be loaded for another reason?
				PlayerState::new(playerid.clone())
			);
		if state.room == Some(purgatory::purgatory_id()){
			state.respawn();
		}
		
		if let Err(_e) = self.add_loaded_player(state.clone()){
			state.room = None;
			state.pos = RoomPos::Unknown;
			if let Err(_e) = self.add_loaded_player(state.clone()) {
				state.room = Some(purgatory::purgatory_id());
				self.add_loaded_player(state)?;
			}
		}
		Ok(())
	}
	
	fn discorporate_player(&mut self, playerid: &PlayerId) -> Result<PlayerState> {
		let roomid = self.players.remove(playerid).ok_or(aerr!("player not found"))?;
		let room = self.get_room_mut(&roomid)?;
		room.remove_player(playerid)
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let player_state = self.discorporate_player(playerid)?;
		self.persistence.save_player(playerid.clone(), player_state)?;
		Ok(())
	}
	
	
	pub fn control_player(&mut self, player: PlayerId, control: Control) -> Result<()>{
		let roomid = self.players.get(&player).ok_or(aerr!("player not found"))?.clone();
		self.get_room_mut(&roomid)?.control_player(player, control);
		Ok(())
	}
	
	fn migrate_player(&mut self, player: &PlayerId, destination: RoomId, roompos: RoomPos) -> Result<()> {
		let mut state = self.discorporate_player(player)?;
		let old_room = state.room;
		state.room = Some(destination);
		state.pos = roompos;
		if let Err(_e) = self.add_loaded_player(state.clone()){
			state.room = old_room;
			state.pos = RoomPos::Unknown;
			if let Err(_e) = self.add_loaded_player(state.clone()) {
				state.room = None;
				if let Err(_e) = self.add_loaded_player(state.clone()) {
					state.room = Some(purgatory::purgatory_id());
					self.add_loaded_player(state)?;
				}
			}
		}
		Ok(())
	}
	
	
	pub fn update(&mut self) {
		self.migrate();
		for room in self.rooms.values_mut() {
			room.update(self.time);
		}
		self.time.0 += 1;
	}
	
	fn migrate(&mut self) {
		let mut migrants = Vec::new();
		for room in self.rooms.values_mut() {
			migrants.append(&mut room.emigrate());
		}
		for (player, destination, roompos) in migrants {
			self.migrate_player(&player, destination, roompos).unwrap();
		}
	}
	
	pub fn save(&self) {
		for room in self.rooms.values() {
			if let Err(err) = self.persistence.save_room(room.id.clone(), room.save()) {
				println!("{:?}",err);
			}
			for (playerid, state) in room.save_players() {
				if let Err(err) = self.persistence.save_player(playerid.clone(), state.clone()) {
					println!("{:?}",err);
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
