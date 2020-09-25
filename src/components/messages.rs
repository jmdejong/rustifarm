
use std::collections::HashMap;
use std::any::Any;
use strum_macros::{EnumString, Display};
use specs::{
	Component,
	DenseVecStorage,
	Entity,
	WriteStorage
};
use super::equipment::Stat;



pub trait Message: Send + Sync + Any + PartialEq {}

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
	
	pub fn has_message(&self, messages: &[M]) -> bool {
		for message in self.messages.iter() {
			for asked in messages {
				if message == asked {
					return true;
				}
			}
		}
		false
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

#[derive(Debug, Clone, PartialEq)]
pub struct AttackMessage {
	pub attacker: Option<Entity>,
	pub typ: AttackType
}

impl Message for AttackMessage {}

pub type AttackInbox = Inbox<AttackMessage>;








#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all="lowercase")]
pub enum Trigger {
	// basic triggers
	Loot,
	Remove,
	Build,
	Spawn,
	// combination triggers
	Die, // Remove + Loot
	Change // Remove + Build
}

impl Message for Trigger {}

pub type TriggerBox = Inbox<Trigger>;


