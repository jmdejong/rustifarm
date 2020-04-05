
use serde_json::{Value};
use specs::{
	Component,
	HashMapStorage
};
use crate::{
	Template
};

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(HashMapStorage)]
pub enum Interactable {
	Harvest,
	Change(Template),
	Say(String),
	Reply(String)
}

use Interactable::*;

impl Interactable {
	pub fn from_json(val: &Value) -> Option<Self> {
		let typ = if val.is_string() {val} else {val.get(0)?};
		let arg = if val.is_string() {&Value::Null} else {val.get(1)?};
		Some(match typ.as_str()? {
			"harvest" => Harvest,
			"change" => Change(Template::from_json(arg).ok()?),
			"say" => Say(arg.as_str()?.to_string()),
			"reply" => Reply(arg.as_str()?.to_string()),
			_ => None?
		})
	}
	
	pub fn accepts_arg(&self, arg: &Option<String>) -> bool {
		match self {
			Harvest => arg.is_none(),
			Change(_) => arg.is_none(),
			Say(_) => arg.is_none(),
			Reply(_) => arg.is_some(),
		}
	}
}
