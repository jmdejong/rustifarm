
use std::any::Any;
use specs::{
	Component,
	DenseVecStorage,
	Entity,
	WriteStorage
};



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

#[derive(Debug, Clone)]
pub struct AttackMessage {
	pub damage: i64,
	pub attacker: Option<Entity>
}

impl AttackMessage {
	pub fn new(damage: i64) -> Self {
		Self {
			damage,
			attacker: None
		}
	}
}

impl Message for AttackMessage {}

pub type AttackInbox = Inbox<AttackMessage>;
