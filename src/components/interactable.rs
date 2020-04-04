
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
	Say(String)
}

impl Interactable {
	pub fn from_json(val: &Value) -> Option<Self> {
		let typ = if val.is_string() {val} else {val.get(0)?};
		let arg = if val.is_string() {&Value::Null} else {val.get(1)?};
		match typ.as_str()? {
			"harvest" => Some(Interactable::Harvest),
			"change" => Some(Interactable::Change(Template::from_json(arg).ok()?)),
			"say" => Some(Interactable::Say(arg.as_str()?.to_string())),
			_ => None
		}
	}
}
