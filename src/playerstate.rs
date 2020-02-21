

use serde_json::{Value, json};
use crate::template::Template;
use crate::{
	componentwrapper::{ComponentWrapper, PreEntity},
	PlayerId,
	RoomId,
	components::{Visible, Player, Inventory, Health, Item},
	Result,
	aerr
};

#[derive(Debug, Clone)]
pub struct PlayerState {
	pub id: PlayerId,
	pub room: Option<RoomId>,
	pub inventory_capacity: usize,
	pub inventory: Vec<Template>,
	pub health: i64,
	pub maximum_health: i64
}

impl PlayerState {

	pub fn new(id: PlayerId) -> Self {
		Self{
			id,
			room: None,
			inventory: Vec::new(),
			inventory_capacity: 10,
			health: 9,
			maximum_health: 10
		}
	}

	pub fn create(id: PlayerId, room: RoomId, inventory: Vec<Template>, inventory_capacity: usize, health: i64, maximum_health: i64) -> Self {
		Self {
			id,
			room: Some(room),
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
				"capacity": self.inventory_capacity,
				"items": self.inventory.iter().map(Template::to_json).collect::<Vec<Value>>()
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
		let mut items = vec![];
		for item in inventory.get("items").ok_or(aerr!("inventory does not have items"))?.as_array().ok_or(aerr!("inventory items not an array"))? {
			items.push(Template::from_json(item)?);
		}
		Ok(Self {
			id: PlayerId{name: val.get("name").ok_or(aerr!("player json does not have name"))?.as_str().ok_or(aerr!("player name not a string"))?.to_string()},
			room: match val.get("roomname").ok_or(aerr!("player json does not have room name"))? {
				Value::String(name) => Some(RoomId::from_str(name)),
				_ => None
			},
			inventory: items,
			health: val.get("health").ok_or(aerr!("player json does not have health"))?.as_i64().ok_or(aerr!("player health not a number"))?,
			inventory_capacity: inventory.get("capacity").ok_or(aerr!("inventory does no have capacity"))?.as_i64().ok_or(aerr!("inventory capacity not a number"))? as usize,
			maximum_health: val.get("maxhealth").ok_or(aerr!("player json does not have maxhealth"))?.as_i64().ok_or(aerr!("maxhealh not a number"))?
		})
	}
	
	pub fn construct(&self) -> PreEntity {
		vec![
			ComponentWrapper::Visible(Visible{sprite: "player".to_string(), height: 1.0}),
			ComponentWrapper::Player(Player::new(self.id.clone())),
			ComponentWrapper::Inventory(Inventory{
				items: self.inventory.iter().map(
					|template| Item{ent: template.clone(), name: template.name.clone()}
				).collect(),
				capacity: self.inventory_capacity
			}),
			ComponentWrapper::Health(Health{health: self.health, maxhealth: self.maximum_health})
		]
	}
}
