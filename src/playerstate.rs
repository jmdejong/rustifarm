

use serde_json::{Value, json};
use crate::template::Template;
use crate::{
	componentwrapper::{ComponentWrapper, PreEntity},
	PlayerId,
	RoomId,
	components::{Visible, Player, Inventory, Health, Item}
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
			id: id,
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

	pub fn from_json(val: &Value) -> Option<Self> {
		let inventory = val.get("inventory")?;
		let mut items = vec![];
		for item in inventory.get("items")?.as_array()? {
			items.push(Template::from_json(item)?);
		}
		Some(Self {
			id: PlayerId{name: val.get("name")?.as_str()?.to_string()},
			room: match val.get("roomname")? {
				Value::String(name) => Some(RoomId::from_str(name)),
				_ => None
			},
			inventory: items,
			health: val.get("health")?.as_i64()?,
			inventory_capacity: inventory.get("capacity")?.as_i64()? as usize,
			maximum_health: val.get("maxhealth")?.as_i64()?
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
