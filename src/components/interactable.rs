
use std::collections::HashMap;
use serde_json;
use serde_json::{Value};
use specs::{
	Component,
	HashMapStorage
};
use crate::{
	exchange::Exchange,
	components::{Trigger, equipment::Stat},
	RoomId
};

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub enum Interactable {
	Trigger(Trigger),
	Visit(RoomId),
	Mine(Stat)
}

use Interactable::*;

impl Interactable {
	pub fn from_json(val: &Value) -> Option<Self> {
		let typ = val.get(0)?;
		let arg = val.get(1)?;
		Some(match typ.as_str()? {
			"trigger" => Trigger(Trigger::from_str(arg.as_str()?)?),
			"visit" => Visit(RoomId::from_str(arg.as_str()?)),
			"mine" => Mine(Stat::from_str(arg.as_str()?)?),
			_ => None?
		})
	}
	
	pub fn accepts_arg(&self, arg: &Option<String>) -> bool {
		match self {
			Trigger(_) => arg.is_none(),
			Visit(_) => {
				if let Some(txt) = arg {
					 txt.starts_with("visit ") || txt.starts_with("disallow ") || txt.starts_with("allow ") || txt.starts_with("whitelist")
				} else {
					true
				}
			}
			Mine(_) => arg.is_none()
		}
	}
}

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct Talkable {
	pub text: String
}


#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub struct Exchanger {
	pub prefix: String,
	pub exchanges: HashMap<String, Exchange>
}

