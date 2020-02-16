

use serde_json::{Value, json};
use crate::template::Template;


pub struct PlayerState {
	name: String,
	room: String,
	inventory_capacity: usize,
	inventory: Vec<Template>,
	health: i64,
	maximum_health: i64
}

impl PlayerState {

	pub fn new(name: String, room: String, inventory: Vec<Template>, health: i64) -> Self {
		Self {
			name,
			room,
			inventory,
			health,
			inventory_capacity: 10,
			maximum_health: 50
		}
	}

	pub fn to_json(&self) -> Value {
		json!({
			"name": self.name,
			"roomname": self.room,
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

	pub fn from_json(val: Value) -> Option<Self> {
		let inventory = val.get("inventory")?;
		let mut items = vec![];
		for item in inventory.get("items")?.as_array()? {
			items.push(Template::from_json(item)?);
		}
		Some(Self {
			name: val.get("name")?.as_str()?.to_string(),
			room: val.get("roomname")?.as_str()?.to_string(),
			inventory: items,
			health: val.get("health")?.as_i64()?,
			inventory_capacity: inventory.get("capacity")?.as_i64()? as usize,
			maximum_health: val.get("maxhealth")?.as_i64()?
		})
	}
}
