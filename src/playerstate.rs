
use std::collections::HashMap;
use serde_json::{Value, json};
use crate::{
	Template,
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
		Equipment,
		equipment::Slot,
		Ear
	},
	Result,
	aerr,
	Sprite,
	Encyclopedia,
	Pos,
	hashmap
};

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
	pub inventory_capacity: usize,
	pub inventory: Vec<(ItemId, bool)>,
	pub health: i64,
	pub maximum_health: i64,
	pub equipment: HashMap<Slot, Option<Template>>
}

impl PlayerState {

	pub fn new(id: PlayerId) -> Self {
		Self{
			id,
			room: None,
			pos: RoomPos::Unknown,
			inventory: Vec::new(),
			inventory_capacity: 10,
			health: 25,
			maximum_health: 50,
			equipment: hashmap!(Slot::Hand => None, Slot::Body => None)
		}
	}

	pub fn create(id: PlayerId, room: RoomId, inventory: Vec<(ItemId, bool)>, inventory_capacity: usize, health: i64, maximum_health: i64, equipment: HashMap<Slot, Option<Template>>) -> Self {
		Self {
			id,
			room: Some(room),
			pos: RoomPos::Unknown,
			inventory,
			health,
			inventory_capacity,
			maximum_health,
			equipment
		}
	}

	pub fn to_json(&self) -> Value {
		json!({
			"name": self.id.name,
			"roomname": match &self.room {
				Some(id) => json!(id.to_string()),
				None => json!(null)
			},
			"inventory": {
				"capacity": self.inventory_capacity,
				"items": self.inventory.iter().map(|(item, e)| (json!(item.0), *e)).collect::<Vec<(Value, bool)>>()
			},
			"equipment": {
				"hand": null,
				"body": null
			},
			"health": self.health,
			"maxhealth": self.maximum_health
		})
	}

	pub fn from_json(val: &Value) -> Result<Self> {
		let inventory = val.get("inventory").ok_or(aerr!("player json does not have inventory"))?;
		let items = 
			inventory
			.get("items")
			.ok_or(aerr!("inventory does not have items"))?
			.as_array()
			.ok_or(aerr!("inventory items not an array"))?
			.into_iter()
			.map(|entry| {
				let itemid = ItemId(
					entry
					.get(0)
					.ok_or(aerr!("item does not have name"))?
					.as_str()
					.ok_or(aerr!("item name not a string"))?
					.to_string()
				);
				let is_equipped =
					entry
					.get(1)
					.ok_or(aerr!("item does not have equipped flag"))?
					.as_bool()
					.ok_or(aerr!("item is_equipped not a bool"))?;
				Ok((itemid, is_equipped))
			})
			.collect::<Result<Vec<(ItemId, bool)>>>()?;
		Ok(Self {
			id: PlayerId{name: val.get("name").ok_or(aerr!("player json does not have name"))?.as_str().ok_or(aerr!("player name not a string"))?.to_string()},
			room: match val.get("roomname").ok_or(aerr!("player json does not have room name"))? {
				Value::String(name) => Some(RoomId::from_str(name)),
				_ => None
			},
			pos: RoomPos::Unknown,
			inventory: items,
			health: val.get("health").ok_or(aerr!("player json does not have health"))?.as_i64().ok_or(aerr!("player health not a number"))?,
			inventory_capacity: inventory.get("capacity").ok_or(aerr!("inventory does no have capacity"))?.as_i64().ok_or(aerr!("inventory capacity not a number"))? as usize,
			maximum_health: val.get("maxhealth").ok_or(aerr!("player json does not have maxhealth"))?.as_i64().ok_or(aerr!("maxhealth not a number"))?,
			equipment: HashMap::new()
		})
	}
	
	pub fn respawn(&mut self) {
		self.room = None;
		self.pos = RoomPos::Unknown;
		self.health = self.maximum_health / 2;
	}
	
	pub fn construct(&self, encyclopedia: &Encyclopedia) -> PreEntity {
		vec![
			ComponentWrapper::Visible(Visible{sprite: Sprite{name: "player".to_string()}, height: 1.2, name: self.id.name.clone()}),
			ComponentWrapper::Player(Player::new(self.id.clone())),
			ComponentWrapper::Inventory(Inventory{
				items: self.inventory.iter().map( |(itemid, is_equipped)| {
					let item = encyclopedia.get_item(itemid).unwrap();
					InventoryEntry{itemid: itemid.clone(), item, is_equipped: *is_equipped}
				}).collect(),
				capacity: self.inventory_capacity
			}),
			ComponentWrapper::Health(Health{health: self.health, maxhealth: self.maximum_health}),
			ComponentWrapper::Fighter(Fighter{attack: AttackType::Attack(5), cooldown: 8, range: 1}),
			ComponentWrapper::Healing(Healing{delay: 50, health: 1, next_heal: None}),
			ComponentWrapper::Movable(Movable{cooldown: 2}),
			ComponentWrapper::Autofight(Autofight::default()),
			ComponentWrapper::Faction(Faction::Good),
			ComponentWrapper::Equipment(Equipment{slots: vec!(Slot::Hand, Slot::Body)}),
			ComponentWrapper::Ear(Ear::default())
		]
	}
}
