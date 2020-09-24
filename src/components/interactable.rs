
use std::collections::HashMap;
use specs::{
	Component,
	HashMapStorage,
};
use crate::{
	exchange::Exchange,
	components::{Trigger, equipment::Stat},
	RoomId,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	ItemId,
};

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub enum Interactable {
	Trigger(Trigger),
	Visit(RoomId),
	Mine(Stat),
	Say(String),
	Reply(String),
	Exchange(String, HashMap<String, Exchange>),
}

use Interactable::*;

impl Interactable {
	
	pub fn parse_from_parameter(typ: &str, arg: &Parameter) -> Option<Self> {
		Some(match (typ, arg) {
			("trigger", Parameter::String(s)) => Trigger(Trigger::from_str(s)?),
			("visit", Parameter::String(s)) => Visit(RoomId(s.clone())),
			("mine", Parameter::String(s)) => Mine(Stat::from_str(s)?),
			("say", Parameter::String(s)) => Say(s.clone()),
			("reply", Parameter::String(s)) => Reply(s.clone()),
			("exchange", p) => {
				let (prefix, trades) = <(String, Vec<(String, Vec<ItemId>, Vec<ItemId>)>)>::from_parameter(p.clone())?;
				let exchanges = trades.into_iter().map(|(k, cost, offer)| (k, Exchange{cost, offer})).collect();
				Exchange(prefix, exchanges)
			}
			_ => {return None}
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
			Mine(_) => arg.is_none(),
			Say(_) => arg.is_none(),
			Reply(_) => arg.is_some(),
			Exchange(prefix, _exchanges) => {
				if let Some(txt) = arg {
					 txt.starts_with(prefix)
				} else {
					true
				}
			},
		}
	}
}


