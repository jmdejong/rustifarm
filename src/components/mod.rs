
pub mod item;
pub mod messages;
pub mod faction;
pub mod interactable;
pub mod equipment;
pub mod inventory;
pub mod serialise;
pub mod flags;

pub use item::Item;
pub use messages::{
	AttackMessage,
	AttackInbox,
	AttackType
};
pub use faction::Faction;
pub use interactable::Interactable;
pub use equipment::Equipment;
pub use inventory::Inventory;
pub use serialise::Serialise;
pub use flags::{
	Flag,
	Flags
};

use specs::{
	DenseVecStorage,
	VecStorage,
	HashMapStorage,
	FlaggedStorage,
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
	Timestamp
};

#[derive(Debug, Clone)]
pub struct Position{
	pub pos: Pos
}
impl Position {
	pub fn new(pos: Pos) -> Position {
		Position{pos}
	}
}

impl Component for Position {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug, Clone)]
pub struct Visible {
	pub sprite: Sprite,
	pub height: f64,
	pub name: String
}
impl Component for Visible {
	type Storage = FlaggedStorage<Self, VecStorage<Self>>;
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

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Dead;

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
pub struct Volatile {
	pub delay: i64,
	pub end_time: Option<Timestamp>
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
	pub homesickness: f64,
	pub view_distance: i64
}

#[derive(Component, Debug, Clone, Default)]
#[storage(HashMapStorage)]
pub struct Home {
	pub home: Pos
}

#[derive(Component, Debug, Clone, Default)]
pub struct Mortal;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Spawner {
	pub amount: usize,
	pub delay: i64,
	pub clan: Clan,
	pub template: Template,
	pub last_spawn: Option<Timestamp>
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
pub struct Grow {
	pub delay: i64,
	pub target_time: Option<Timestamp>,
	pub into: Template
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct CreationTime {
	pub time: Timestamp
}


