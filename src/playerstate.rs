
use serde_json::{Value, json};
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
		Equipment,
		equipment::Slot,
		Ear
	},
	Result,
	aerr,
	Sprite,
	Encyclopedia,
	Pos,
	PResult,
	perr
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
	pub maximum_health: i64
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
			maximum_health: 50
		}
	}

	pub fn create(id: PlayerId, room: RoomId, inventory: Vec<(ItemId, bool)>, inventory_capacity: usize, health: i64, maximum_health: i64) -> Self {
		Self {
			id,
			room: Some(room),
			pos: RoomPos::Unknown,
			inventory,
			health,
			inventory_capacity,
			maximum_health
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
				"items": self.inventory.iter().map(|(item, e)| (json!(item.0), *e)).collect::<Vec<(Value, bool)>>()
			},
			"health": self.health
		})
	}

	pub fn from_json(val: &Value) -> PResult<Self> {
		let inventory = val.get("inventory").ok_or(perr!("player json does not have inventory"))?;
		let mut items = 
			inventory
			.get("items")
			.ok_or(perr!("inventory does not have items"))?
			.as_array()
			.ok_or(perr!("inventory items not an array"))?
			.iter()
			.map(|entry| {
				if entry.is_array() {
					let itemid = ItemId(
						entry
						.get(0)
						.ok_or(perr!("item does not have name"))?
						.as_str()
						.ok_or(perr!("item name not a string"))?
						.to_string()
					);
					let is_equipped =
						entry
						.get(1)
						.ok_or(perr!("item does not have equipped flag"))?
						.as_bool()
						.ok_or(perr!("item is_equipped not a bool"))?;
					Ok((itemid, is_equipped))
				} else if entry.is_string() {
					Ok((ItemId(entry.as_str().unwrap().to_string()), false))
				} else {
					Err(perr!("item entry must be a string or array, not {:?}", entry))
				}
			})
			.collect::<PResult<Vec<(ItemId, bool)>>>()?;
		if let Some(equipment) = val.get("equipment") {
			for (slot, item) in equipment.as_object().ok_or(perr!("equipment not a json object: {:?}", equipment))?.iter() {
				if item.is_null(){
					continue
				}
				let itemid = ItemId(
					item
					.as_str()
					.ok_or(perr!("equipment item not a string: {:?}", item))?
					.to_string()
				);
				// validate the slot, but don't do anything with it
				Slot::from_str(slot).ok_or(perr!("invalid slot: {:?}", slot))?;
				items.push((itemid, true))
			}
		}
		Ok(Self {
			id: PlayerId{name: val.get("name").ok_or(perr!("player json does not have name"))?.as_str().ok_or(perr!("player name not a string"))?.to_string()},
			room: match val.get("roomname").ok_or(perr!("player json does not have room name"))? {
				Value::String(name) => Some(RoomId::from_str(name)),
				_ => None
			},
			pos: RoomPos::Unknown,
			inventory: items,
			health: val.get("health").ok_or(perr!("player json does not have health"))?.as_i64().ok_or(perr!("player health not a number"))?,
			inventory_capacity: 12,
			maximum_health: 50,
		})
	}
	
	pub fn respawn(&mut self) {
		self.room = None;
		self.pos = RoomPos::Unknown;
		self.health = self.maximum_health / 2;
	}
	
	pub fn construct(&self, encyclopedia: &Encyclopedia) -> Result<PreEntity> {
		Ok(vec![
			ComponentWrapper::Visible(Visible{sprite: Sprite{name: "player".to_string()}, height: 1.75, name: self.id.name.clone()}),
			ComponentWrapper::Player(Player::new(self.id.clone())),
			ComponentWrapper::Inventory(Inventory{
				items: self.inventory.iter().map( |(itemid, is_equipped)| {
						let item = encyclopedia.get_item(&itemid).ok_or(aerr!("failed to load item '{:?} in inventory of player {:?}", itemid, self))?;
					Ok(InventoryEntry{itemid: itemid.clone(), item, is_equipped: *is_equipped})
				}).collect::<Result<Vec<InventoryEntry>>>()?,
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
		])
	}
}
