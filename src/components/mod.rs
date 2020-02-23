
pub mod item;

pub use item::Item;

use specs::{
	DenseVecStorage,
	VecStorage,
	HashMapStorage,
	FlaggedStorage,
	NullStorage,
	Component,
	Entity,
	WriteStorage
};

use crate::{
	Pos,
	PlayerId,
	RoomId,
	Sprite,
	controls::Control,
	Template,
	playerstate::RoomPos,
	attack::Attack
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
pub struct Controller(pub Control);

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Blocking;

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Floor;

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

#[derive(Debug, Clone, Default)]
pub struct Inventory {
	pub items: Vec<Item>,
	pub capacity: usize
}
impl Component for Inventory {
	type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
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
pub struct Serialise {
	pub template: Template
}

#[derive(Component, Debug, Clone)]
pub struct RoomExit {
	pub destination: RoomId,
	pub dest_pos: RoomPos
}

#[derive(Component, Debug, Clone, Default)]
pub struct Attacked {
	pub attacks: Vec<Attack>
}

pub fn add_attack(attacked: &mut WriteStorage<Attacked> , ent: Entity, attack: Attack) {
	attacked
		.entry(ent)
		.unwrap()
		.or_insert_with(Attacked::default)
		.attacks
		.push(attack);
}


#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Entered;

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Dying;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Trap {
	pub attack: Attack
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Fighter {
	pub attack: Attack
}

