
use std::collections::HashMap;

use specs::Dispatcher;

use crate::{
	PlayerId,
	RoomId,
	room::Room,
	room,
	worldloader::WorldLoader,
	persistence::{PersistentStorage, LoaderError},
	playerstate::{PlayerState, RoomPos},
	Encyclopedia,
	controls::Control,
	errors::{AnyError},
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
	room_age: HashMap<RoomId, Timestamp>,
	encyclopedia: Encyclopedia,
	pub time: Timestamp,
	default_dispatcher: Dispatcher<'a, 'b>
}

#[derive(Debug)]
pub enum MigrationError {
	PlayerError(AnyError),
	RoomError(AnyError)
}

impl <'a, 'b>World<'a, 'b> {
	
	pub fn new(encyclopedia: Encyclopedia, template_loader: WorldLoader, persistence: Box<dyn PersistentStorage>, default_room: RoomId) -> Self {
		let time = match persistence.load_world_meta() {
			Ok(time) => {time}
			Err(LoaderError::MissingResource(_)) => {
				Timestamp(1000000)
			}
			Err(LoaderError::InvalidResource(err)) => {
				panic!("Invalid world meta: {:?}", err)
			}
		};
		World {
			template_loader,
			time,
			persistence,
			default_room,
			encyclopedia: encyclopedia,
			players: HashMap::new(),
			rooms: HashMap::new(),
			room_age: HashMap::new(),
			default_dispatcher: room::default_dispatcher()
		}
	}
	
	fn get_room_mut(&mut self, id: &RoomId) -> Result<&mut Room<'a, 'b>> {
		self.room_age.insert(id.clone(), self.time);
		let result = self.get_room_mut_(id);
		if let Err(err) = &result {
			println!("Failed to load room {:?}: {:?}", id, err);
		}
		result
	}
	
	fn get_room_mut_(&mut self, id: &RoomId) -> Result<&mut Room<'a, 'b>> {
		if !self.rooms.contains_key(id){
			println!("loading room '{}'", id.name);
			let mut room: Room = if id == &purgatory::purgatory_id() {
					purgatory::create_purgatory(&self.encyclopedia)
				} else {
					let mut room = Room::new(id.clone(), self.encyclopedia.clone(), None);
					let template = self.template_loader.load_room(id.clone())?;
					room.load_from_template(&template)?;
					room
				};
			match self.persistence.load_room(id.clone()){
				Ok(state) => {
					room.load_saved(&state);
				}
				Err(LoaderError::MissingResource(_)) => {}
				Err(LoaderError::InvalidResource(err)) => {return Err(err);}
			}
			let last_time = self.time - 1;
			if room.get_time() < last_time {
				room.update(last_time, &mut self.default_dispatcher);
			}
			self.rooms.insert(id.clone(), room);
		}
		Ok(self.rooms.get_mut(id).expect("can't get room after loading it"))
	}
	
	fn add_loaded_player(&mut self, state: PlayerState) -> std::result::Result<(), MigrationError> {
		let roomid = state.clone().room.unwrap_or_else(|| self.default_room.clone());
		let room = self.get_room_mut(&roomid).map_err(|e| MigrationError::RoomError(e))?;
		room.add_player(&state).map_err(|e| MigrationError::PlayerError(e))?;
		self.players.insert(state.id, roomid);
		Ok(())
	}
	
	fn try_add_loaded_player(&mut self, mut state: PlayerState, backups: &[Option<RoomId>]) -> Result<()> {
		match self.add_loaded_player(state.clone()){
			Err(MigrationError::RoomError(e)) => {
				println!("could not add player {:?} to room {:?}: {:?}", state.id, state.room, e);
				if let Some((first, rest)) = backups.split_first(){
					state.room = first.clone();
					state.pos = RoomPos::Unknown;
					self.try_add_loaded_player(state, rest)
				} else {
					Err(e)
				}
			}
			Err(MigrationError::PlayerError(e)) => {
				println!("could not load player {:?} to room {:?}: {:?}", state.id, state.room, e);
				Err(e)
			}
			Ok(()) => Ok(())
		}
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let mut state = match self.persistence.load_player(playerid.clone()) {
			Ok(state) => {state}
			Err(LoaderError::MissingResource(_)) => {
				PlayerState::new(playerid.clone())
			}
			Err(LoaderError::InvalidResource(err)) => {
				return Err(err)
			}
		};
		state.id = playerid.clone();
		if state.room == Some(purgatory::purgatory_id()){
			state.respawn();
		}
		
		self.try_add_loaded_player(state, &[None])
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
		self.try_add_loaded_player(state, &[old_room, None])
	}
	
	
	pub fn update(&mut self) {
		self.migrate();
		for room in self.rooms.values_mut() {
			room.update(self.time, &mut self.default_dispatcher);
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
		if let Err(err) = self.persistence.save_world_meta(self.time) {
			println!("{:?}",err);
		}
	}
	
	pub fn unload_rooms(&mut self, min_age: i64){
		let mut to_remove = Vec::new();
		for roomid in self.rooms.keys() {
			if self.rooms[roomid].has_players() {
				self.room_age.insert(roomid.clone(), self.time);
			} else {
				let age = self.time - *self.room_age.get(&roomid).unwrap_or(&Timestamp(0));
				if age >= min_age {
					to_remove.push(roomid.clone());
				}
			}
		}
		for roomid in to_remove {
			println!("unloading room '{}'", roomid.name);
			self.rooms.remove(&roomid);
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

