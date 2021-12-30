
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::{
	componentwrapper::{ComponentWrapper, PreEntity},
	PlayerId,
	RoomId,
	ItemId,
	components::{
		Visible,
		Player,
		Inventory,
		inventory::InventoryEntry,
		Health,
		Fighter,
		Healing,
		Movable,
		AttackType,
		Autofight,
		Faction,
		Slot,
		Ear,
		Stats,
		Stat,
		Description
	},
	Result,
	aerr,
	Sprite,
	Encyclopedia,
	Pos,
	hashmap
};

#[allow(non_upper_case_globals)]
const maximum_health: i64 = 50;
#[allow(non_upper_case_globals)]
const inventory_capacity: usize = 20;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RoomPos {
	Pos(Pos),
	Name(String),
	Unknown
}

#[derive(Debug, Clone)]
pub struct PlayerState {
	pub id: PlayerId,
	pub room: Option<RoomId>,
	pub pos: RoomPos,
	pub inventory: Vec<(ItemId, bool)>,
	pub health: i64
}

impl PlayerState {

	pub fn new(id: PlayerId) -> Self {
		Self{
			id,
			room: None,
			pos: RoomPos::Unknown,
			inventory: Vec::new(),
			health: maximum_health/2,
		}
	}

	pub fn create(id: PlayerId, room: RoomId, inventory: Vec<(ItemId, bool)>, health: i64) -> Self {
		Self {
			id,
			room: Some(room),
			pos: RoomPos::Unknown,
			inventory,
			health,
		}
	}
	
	pub fn respawn(&mut self) {
		self.room = None;
		self.pos = RoomPos::Unknown;
		self.health = maximum_health / 2;
	}
	
	pub fn construct(&self, encyclopedia: &Encyclopedia) -> Result<PreEntity> {
		Ok(vec![
			ComponentWrapper::Visible(Visible{sprite: Sprite("player".to_string()), height: 1.75, name: self.id.0.clone()}),
			ComponentWrapper::Player(Player::new(self.id.clone())),
			ComponentWrapper::Inventory(Inventory{
				items: self.inventory.iter().map( |(itemid, is_equipped)| {
						let item = encyclopedia.get_item(&itemid).ok_or(aerr!("failed to load item '{:?} in inventory of player {:?}", itemid, self))?;
					Ok(InventoryEntry{itemid: itemid.clone(), item, is_equipped: *is_equipped})
				}).collect::<Result<Vec<InventoryEntry>>>()?,
				capacity: inventory_capacity
			}),
			ComponentWrapper::Health(Health{health: self.health, maxhealth: maximum_health}),
			ComponentWrapper::Fighter(Fighter{attack: AttackType::Attack(5), cooldown: 8, range: 1}),
			ComponentWrapper::Healing(Healing{delay: 50, health: 1, next_heal: None}),
			ComponentWrapper::Movable(Movable{cooldown: 2}),
			ComponentWrapper::Autofight(Autofight::default()),
			ComponentWrapper::Faction(Faction::Good),
			ComponentWrapper::Ear(Ear::default()),
			ComponentWrapper::Stats(Stats{skills: hashmap!{Stat::Gathering => 10}}),
			ComponentWrapper::Description(Description{description: format!("a player named {}", self.id.0)})
		])
	}
}

impl Serialize for PlayerState {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where S: Serializer {
		PlayerStateSave::New{name: self.id.clone(), roomname: self.room.clone(), inventory: NewInventorySave{items: self.inventory.clone()}, health: self.health}.serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for PlayerState {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where D: Deserializer<'de> {
		Ok(match PlayerStateSave::deserialize(deserializer)? {
			PlayerStateSave::New{name, roomname, inventory, health} => PlayerState{id: name, room: roomname, inventory: inventory.items, health, pos: RoomPos::Unknown},
			PlayerStateSave::Old{name, roomname, inventory, equipment, health} => {
				PlayerState{
					id: name,
					room: roomname,
					inventory: {
						let mut inv = Vec::new();
						for item in inventory.items {
							inv.push((item, false));
						}
						for (_slot, item) in equipment.into_iter() {
							inv.push((item, true));
						}
						inv
					},
					health,
					pos: RoomPos::Unknown
				}
			}
		})
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct OldInventorySave {
	pub items: Vec<ItemId>
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NewInventorySave {
	pub items: Vec<(ItemId, bool)>
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)] 
enum PlayerStateSave {
	New {
		name: PlayerId,
		roomname: Option<RoomId>,
		inventory: NewInventorySave,
		health: i64
	},
	Old {
		name: PlayerId,
		roomname: Option<RoomId>,
		inventory: OldInventorySave,
		equipment: HashMap<Slot, ItemId>,
		health: i64
	}
}
