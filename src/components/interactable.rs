
use std::collections::HashMap;
use serde_json;
use serde_json::{Value};
use specs::{
	Component,
	HashMapStorage
};
use crate::{
	exchange::Exchange,
	ItemId,
	components::{Trigger, equipment::Stat},
	RoomId
};

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub enum Interactable {
	Trigger(Trigger),
	Exchange(String, HashMap<String, Exchange>),
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
			"exchange" => {
				let (prefix, change) = serde_json::value::from_value::<
						(String, HashMap<String, (Vec<ItemId>, Vec<ItemId>)>)
					>(arg.clone()).ok()?;
				Exchange(
					prefix,
					change.into_iter().map(
						|(id, (cost, offer))| (id, Exchange{cost, offer})
					).collect::<HashMap<String, Exchange>>()
				)
			},
			"visit" => Visit(RoomId::from_str(arg.as_str()?)),
			"mine" => Mine(Stat::from_str(arg.as_str()?)?),
			_ => None?
		})
	}
	
	pub fn accepts_arg(&self, arg: &Option<String>) -> bool {
		match self {
			Trigger(_) => arg.is_none(),
			Exchange(prefix, _exchanges) => {
				if let Some(txt) = arg {
					 txt.starts_with(prefix)
				} else {
					true
				}
			},
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
