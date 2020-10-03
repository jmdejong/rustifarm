
use std::collections::HashMap;

use specs::{
	World,
	WorldExt,
	Join,
	Entity,
	RunNow
};

use crate::{
	controls::Control,
	worldmessages::WorldMessage,
	resources::{
		Size,
		Output,
		Input,
		NewEntities,
		Spawn as SpawnPosition,
		Players,
		Emigration,
		Time,
		RoomFlags
	},
	components::{
		Position,
		Serialise,
		Player,
		Inventory,
		Health,
		Removed,
		Clan
	},
	Encyclopedia,
	roomtemplate::RoomTemplate,
	savestate::SaveState,
	Template,
	playerstate::{PlayerState, RoomPos},
	componentwrapper::extract_parameter,
	Pos,
	PlayerId,
	RoomId,
	aerr,
	Result,
	Timestamp,
	systems::{
		Move,
		RegisterNew,
		ControlInput,
		View,
		Remove,
		Create,
		Take,
		Migrate,
		Use,
		Attacking,
		Trapping,
		Fight,
		Heal,
		UpdateCooldowns,
		ControlAI,
		Die,
		Spawn,
		Interact,
		DropLoot,
		Timeout,
		Clear,
		Building,
		SpawnTrigger,
		Replace,
		SpawnCheck,
	}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
	Normal,
	Purgatory
}


pub struct Room {
	world: World,
	pub id: RoomId,
	places: HashMap<String, Pos>,
	room_type: RoomType
}

macro_rules! register_insert {
	($world: expr, ($($comp: ident),*), ($($res: ident),*)) => {
		$(
			$world.register::<crate::components::$comp>();
		)*
		$(
			$world.insert(crate::resources::$res::default());
		)*
	}
}


impl Room {

	pub fn new(id: RoomId, encyclopedia: Encyclopedia, room_type: RoomType) -> Room {
		let mut world = World::new();
		world.insert(NewEntities::new(encyclopedia));
		register_insert!(
			world,
			(Position, Visible, Controller, Movable, New, Removed, Moved, Player, Inventory, Health, Serialise, RoomExit, Entered, TriggerBox, Trap, Fighter, Healing, ControlCooldown, Autofight, MonsterAI, AttackInbox, Item, Spawner, Clan, Faction, Interactable, Loot, Timer, TimeOffset, Flags, Ear, Build, Whitelist, Minable, LootHolder, OnSpawn, Substitute, Stats, Requirements),
			(Ground, Input, Output, Size, Spawn, Players, Emigration, Time, RoomFlags)
		);
		
		Room {
			world,
			id,
			places: HashMap::new(),
			room_type
		}
	}
	
	pub fn load_from_template(&mut self, template: &RoomTemplate) -> Result<()> {
	
		let (width, height) = template.size;
		self.world.fetch_mut::<Size>().width = width;
		self.world.fetch_mut::<Size>().height = height;
		
		self.world.fetch_mut::<SpawnPosition>().pos = template.spawn;
		self.world.insert::<RoomFlags>(template.flags.clone());
		
		for (idx, templates) in template.field.iter().enumerate() {
			let x = (idx as i64) % width;
			let y = (idx as i64) / width;
			
			for template in templates {
				self.create_entity(template.clone().unsaved(), Pos{x, y})?;
			}
		}
		for (name, place) in &template.places {
			self.places.insert(name.clone(), *place);
		}
		Ok(())
	}
	
	pub fn view(&self) -> HashMap<PlayerId, WorldMessage> {
		self.world.fetch::<Output>().output.clone()
	}
	
	pub fn update(&mut self, timestamp: Timestamp) {
		self.world.fetch_mut::<Time>().time = timestamp;
		match self.room_type {
			RoomType::Normal => {
				Replace.run_now(&self.world);
				Timeout.run_now(&self.world);
				UpdateCooldowns.run_now(&self.world);
				Spawn.run_now(&self.world);
				SpawnCheck.run_now(&self.world);
				ControlInput.run_now(&self.world);
				ControlAI.run_now(&self.world);
				Take.run_now(&self.world);
				Use.run_now(&self.world);
				Interact.run_now(&self.world);
				SpawnTrigger.run_now(&self.world);
				Move.run_now(&self.world);
				Trapping.run_now(&self.world);
				Fight.run_now(&self.world);
				Heal.run_now(&self.world);
				Attacking.run_now(&self.world);
				Die.run_now(&self.world);
				DropLoot.run_now(&self.world);
				Building.run_now(&self.world);
				Migrate.run_now(&self.world);
			}
			RoomType::Purgatory => {
				UpdateCooldowns.run_now(&self.world);
				ControlInput.run_now(&self.world);
				Move.run_now(&self.world);
			}
		}
		Create.run_now(&self.world);
		Remove.run_now(&self.world);
		self.world.maintain();
		RegisterNew.run_now(&self.world);
		View.run_now(&self.world);
		Clear.run_now(&self.world);
	}
	
	
	pub fn control_player(&mut self, player: PlayerId, control: Control){
		self.world.fetch_mut::<Input>().actions.insert(player, control);
	}
	
	pub fn add_player(&mut self, state: &PlayerState) -> Result<()> {
		let pre_player = state.construct(&self.world.fetch::<NewEntities>().encyclopedia)?;
		let spawn = match &state.pos {
			RoomPos::Unknown => self.world.fetch::<SpawnPosition>().pos,
			RoomPos::Pos(pos) => *pos,
			RoomPos::Name(name) => *self.places.get(name).unwrap()
		};
		self.world.fetch_mut::<NewEntities>().to_build.push((spawn, pre_player));
		Ok(())
	}
	
	pub fn remove_player(&mut self, id: &PlayerId) -> Result<PlayerState>{
		let ent = self.world.fetch_mut::<Players>().entities.remove(id).ok_or(aerr!("failed to remove player"))?;
		let state = self.save_player_ent(ent).ok_or(aerr!("failed to find player to remove"))?;
		self.world.write_component::<Removed>().insert(ent, Removed)?;
		self.world.write_component::<Player>().remove(ent);
		Ok(state)
	}
	
	pub fn save(&self) -> SaveState {
		let entities = self.world.entities();
		let positions = self.world.read_component::<Position>();
		let serialisers = self.world.read_component::<Serialise>();
		let clans = self.world.read_component::<Clan>();
		let mut state = SaveState::new();
		for (entity, pos, serialiser, clan) in (&entities, &positions, &serialisers, (&clans).maybe()).join() {
			let mut template = serialiser.template.clone();
			for (argument, component, member) in &serialiser.extract {
				if let Some(parameter) = extract_parameter(*component, member.as_str(), &self.world, entity){
					template.kwargs.insert(argument.clone(), parameter);
				} else {
					println!("failed to extract parameter {:?} from {:?}", member, component);
				}
			}
			template.clan = clan.map(|c| c.name.clone());
			state.changes.entry(pos.pos).or_insert_with(Vec::new).push(template);
		}
		state
	}
	
	pub fn load_saved(&mut self, state: &SaveState) {
		for (pos, templates) in state.changes.iter() {
			for template in templates {
				self.create_entity(template.clone(), *pos).unwrap();
			}
		}
	}
	
	pub fn has_players(&self) -> bool {
		!self.world.read_component::<Player>().is_empty()
	}
	
	pub fn save_players(&self) -> HashMap<PlayerId, PlayerState> {
		let mut states = HashMap::new();
		let players = self.world.read_component::<Player>();
		let entities = self.world.entities();
		for (ent, player) in (&entities, &players).join(){
			states.insert(player.id.clone(), self.save_player_ent(ent).unwrap());
		}
		states
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
			inventory.items.iter().map(|entry| (entry.itemid.clone(), entry.is_equipped)).collect(),
			health.health,
		))
	}
	
	fn create_entity(&mut self, template: Template, pos: Pos) -> Result<()>{
		self.world.fetch_mut::<NewEntities>().create(pos, &template)?;
		Ok(())
	}
	
	pub fn emigrate(&mut self) -> Vec<(PlayerId, RoomId, RoomPos)> {
		let emigrants = self.world.remove::<Emigration>().expect("World does not have Emigration resource").emigrants;
		self.world.insert(Emigration::default());
		emigrants
	}
	
	pub fn get_time(&self) -> Timestamp {
		self.world.fetch::<Time>().time
	}
}


#[cfg(test)]
mod tests {



}


