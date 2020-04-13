
use std::collections::HashMap;
use std::any::Any;
use specs::{
	Component,
	DenseVecStorage,
	Entity,
	WriteStorage
};
use super::equipment::Stat;



pub trait Message: Send + Sync + Any {}

#[derive(Debug, Clone, Default)]
pub struct Inbox<M: Message> {
	pub messages: Vec<M>
}

impl <M: Message> Component for Inbox<M> {
	type Storage = DenseVecStorage<Self>;
}

impl <M: Message> Inbox<M> {
	
	pub fn add_message(messages: &mut WriteStorage<Self>, ent: Entity, message: M){
		messages
			.entry(ent)
			.unwrap()
			.or_insert_with(|| Self{messages: Vec::new()})
			.messages
			.push(message);
	}
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackType {
	Attack(i64),
	Heal(i64)
}

impl AttackType {
	pub fn is_hostile(&self) -> bool {
		match self {
			Self::Attack(_) => true,
			Self::Heal(_) => false
		}
	}
	pub fn apply_bonuses(self, bonuses: &HashMap<Stat, i64>) -> AttackType {
		match self {
			Self::Attack(strength) => Self::Attack(strength + *bonuses.get(&Stat::Strength).unwrap_or(&0)),
			Self::Heal(_) => self
		}
	}
}

#[derive(Debug, Clone)]
pub struct AttackMessage {
	pub attacker: Option<Entity>,
	pub typ: AttackType
}

impl Message for AttackMessage {}

pub type AttackInbox = Inbox<AttackMessage>;








#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
	Loot,
	Die,
	Remove
}

impl Trigger {
	pub fn from_str(txt: &str) -> Option<Self> {
		Some(match txt {
			"loot" => Self::Loot,
			"die" => Self::Die,
			"remove" => Self::Remove,
			_ => {return None}
		})
	}
}

impl Message for Trigger {}

pub type TriggerBox = Inbox<Trigger>;


