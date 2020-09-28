
pub mod messages;
pub mod faction;
pub mod interactable;
pub mod equipment;
pub mod inventory;
pub mod serialise;
pub mod flags;
pub mod ear;

pub use messages::{
	AttackMessage,
	AttackInbox,
	AttackType,
	Trigger,
	TriggerBox
};
pub use faction::Faction;
pub use interactable::{Interactable};
pub use equipment::Equipment;
pub use inventory::Inventory;
pub use serialise::Serialise;
pub use flags::{
	Flag,
	Flags
};
pub use ear::{
	Notification,
	Ear
};

use std::collections::{HashMap, HashSet};

use specs::{
	DenseVecStorage,
	VecStorage,
	HashMapStorage,
	NullStorage,
	Component,
	Entity
};

use crate::{
	Pos,
	PlayerId,
	RoomId,
	Sprite,
	controls::Control,
	Template,
	playerstate::RoomPos,
	Timestamp,
	ItemId,
};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position{
	pub pos: Pos
}
impl Position {
	pub fn new(pos: Pos) -> Position {
		Position{pos}
	}
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Visible {
	pub sprite: Sprite,
	pub height: f64,
	pub name: String
}

#[derive(Component, Debug)]
pub struct Controller {
	pub control: Control
}

#[derive(Default, Component, Debug, Clone)]
pub struct Movable {
	pub cooldown: i64
}

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct New;

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Removed;

#[derive(Default, Component, Debug, Clone)]
pub struct Moved {
	pub from: Pos
}

#[derive(Default, Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Player {
	pub id: PlayerId
}
impl Player {
	pub fn new(id: PlayerId) -> Self {
		Self{id}
	}
}

#[derive(Component, Debug, Clone)]
pub struct Health {
	pub health: i64,
	pub maxhealth: i64
}
impl Health {
	pub fn heal(&mut self, amount: i64) {
		self.health += amount;
		if self.health > self.maxhealth {
			self.health = self.maxhealth;
		}
	}
}

#[derive(Component, Debug, Clone)]
pub struct RoomExit {
	pub destination: RoomId,
	pub dest_pos: RoomPos
}

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Entered;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Trap {
	pub attack: AttackType
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Fighter {
	pub attack: AttackType,
	pub cooldown: i64,
	pub range: i64
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Healing {
	pub delay: i64,
	pub health: i64,
	pub next_heal: Option<Timestamp>
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct ControlCooldown {
	pub amount: i64
}


#[derive(Component, Debug, Clone, Default)]
#[storage(HashMapStorage)]
pub struct Autofight {
	pub target: Option<Entity>
}

#[derive(Component, Debug, Clone, Default)]
#[storage(HashMapStorage)]
pub struct MonsterAI {
	pub move_chance: f64,
	pub view_distance: i64,
	pub homesickness: f64,
}

#[derive(Component, Debug, Clone, Default)]
#[storage(HashMapStorage)]
pub struct Home {
	pub home: Pos
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Spawner {
	pub amount: usize,
	pub clan: Clan,
	pub template: Template,
	pub saturated: bool,
	pub radius: i64
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
#[storage(HashMapStorage)]
pub struct Clan {
	pub name: String,
}


#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Loot {
	pub loot: Vec<(Template, f64)>
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct LootHolder {
	pub loot: Vec<(Template, f64)>
}



#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Timer {
	pub delay: i64,
	pub spread: f64,
	pub target_time: Option<Timestamp>,
	pub trigger: Trigger
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct TimeOffset {
	pub dtime: i64
}


#[derive(Component, Debug, Clone)]
pub struct Item(pub ItemId);


#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Build {
	pub obj: Template
}

#[derive(Component, Debug, Clone)]
pub struct Whitelist{
	pub allowed: HashMap<String, HashSet<PlayerId>>
}

#[derive(Component, Debug, Clone)]
pub struct Minable {
	pub progress: i64,
	pub total: i64,
	pub trigger: Trigger
}

#[derive(Component, Debug, Clone)]
pub struct OnSpawn {
	pub trigger: Trigger
}


#[derive(Component, Debug, Clone)]
pub struct Substitute {
	pub into: Template
}

