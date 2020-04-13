
use std::collections::HashMap;
use serde_json::{Value};
use specs::{
	Component,
	HashMapStorage
};
use crate::{
	exchange::Exchange,
	ItemId,
	components::Trigger
};

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub enum Interactable {
	Trigger(Trigger),
	Say(String),
	Reply(String),
	Exchange(String, HashMap<String, Exchange>)
}

use Interactable::*;

impl Interactable {
	pub fn from_json(val: &Value) -> Option<Self> {
		let typ = val.get(0)?;
		let arg = val.get(1)?;
		Some(match typ.as_str()? {
			"trigger" => Trigger(Trigger::from_str(arg.as_str()?)?),
			"say" => Say(arg.as_str()?.to_string()),
			"reply" => Reply(arg.as_str()?.to_string()),
			"exchange" => Exchange(
				arg.get(0)?.as_str()?.to_string(),
				arg.get(1)?
					.as_object()?
					.iter()
					.map(|(id, ex)| {
						let exchange = Exchange {
							cost: ex.get(0)?.as_array()?.iter().map(|i| Some(ItemId(i.as_str()?.to_string()))).collect::<Option<Vec<ItemId>>>()?,
							offer: ex.get(1)?.as_array()?.iter().map(|i| Some(ItemId(i.as_str()?.to_string()))).collect::<Option<Vec<ItemId>>>()?
						};
						Some((id.clone(), exchange)) 
					})
					.collect::<Option<HashMap<String, Exchange>>>()?
			),
			_ => None?
		})
	}
	
	pub fn accepts_arg(&self, arg: &Option<String>) -> bool {
		match self {
			Trigger(_) => arg.is_none(),
			Say(_) => arg.is_none(),
			Reply(_) => arg.is_some(),
			Exchange(prefix, _exchanges) => {
				if let Some(txt) = arg {
					 txt.starts_with(prefix)
				} else {
					true
				}
			}
		}
	}
}
