
use std::collections::HashMap;

use specs::{
	World,
	WorldExt,
	DispatcherBuilder,
	Dispatcher,
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
		Time
	},
	components::{
		Position,
		Serialise,
		Player,
		Inventory,
		Health,
		Removed
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
		Volate,
		UpdateCooldowns,
		ControlAI,
		Die,
		Spawn,
		Interact,
		DropLoot,
		Growth,
		Clear,
		Building
	}
};

pub fn default_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
	DispatcherBuilder::new()
		.with(Volate, "volate", &[])
		.with(Growth, "growth", &[])
		.with(UpdateCooldowns, "cool_down", &[])
		.with(Spawn, "spawn", &[])
		.with(ControlInput, "controlinput", &["cool_down"])
		.with(ControlAI, "controlai", &["cool_down"])
		.with(Take, "take", &["controlinput", "controlai"])
		.with(Use, "use", &["controlinput", "controlai"])
		.with(Interact, "interact", &["controlinput", "controlai"])
		.with(Move, "move", &["controlinput", "controlai"])
		.with(Trapping, "trapping", &["move"])
		.with(Fight, "fight", &["move"])
		.with(Heal, "heal", &[])
		.with(Attacking, "attacking", &["use", "trapping", "fight", "heal", "interact"])
		.with(Die, "die", &["attacking"])
		.with(DropLoot, "droploot", &["attacking"])
		.with(Building, "building", &["attacking"])
		.with(Migrate, "migrate", &["move", "attacking", "volate", "die"])
		.build()
}

pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Option<Dispatcher<'a, 'b>>,
	pub id: RoomId,
	places: HashMap<String, Pos>
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


impl <'a, 'b>Room<'a, 'b> {

	pub fn new(id: RoomId, encyclopedia: Encyclopedia, dispatcher: Option<Dispatcher<'a, 'b>>) -> Room<'a, 'b> {
		let mut world = World::new();
		world.insert(NewEntities::new(encyclopedia));
		register_insert!(
			world,
			(Position, Visible, Controller, Movable, New, Removed, Moved, Player, Inventory, Health, Serialise, RoomExit, Entered, TriggerBox, Trap, Fighter, Healing, Volatile, ControlCooldown, Autofight, MonsterAI, Home, AttackInbox, Item, Spawner, Clan, Faction, Interactable, Loot, Grow, Equipment, OwnTime, Flags, Ear, Build), 
			(Ground, Input, Output, Size, Spawn, Players, Emigration, Time)
		);	
		
		Room {
			world,
			dispatcher,
			id,
			places: HashMap::new()
		}
	}
	
	pub fn load_from_template(&mut self, template: &RoomTemplate) -> Result<()> {
	
		let (width, height) = template.size;
		self.world.fetch_mut::<Size>().width = width;
		self.world.fetch_mut::<Size>().height = height;
		
		self.world.fetch_mut::<SpawnPosition>().pos = template.spawn;
		
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
	
	pub fn update(&mut self, timestamp: Timestamp, default_dispatcher: &mut Dispatcher) {
		self.world.fetch_mut::<Time>().time = timestamp;
		if let Some(dispatcher) = &mut self.dispatcher {
			dispatcher.dispatch(&self.world);
		} else {
			default_dispatcher.dispatch(&self.world);
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
		let mut state = SaveState::new();
		for (entity, pos, serialiser) in (&entities, &positions, &serialisers).join() {
			let mut template = serialiser.template.clone();
			for (argument, component, member) in &serialiser.extract {
				if let Some(parameter) = extract_parameter(*component, member.as_str(), &self.world, entity){
					template.kwargs.insert(argument.clone(), parameter);
				} else {
					println!("failed to extract parameter {:?} from {:?}", member, component);
				}
			}
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
			inventory.capacity,
			health.health,
			health.maxhealth
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


