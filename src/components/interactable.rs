
use specs::{
	Component,
	HashMapStorage
};

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(HashMapStorage)]
pub enum Interactable {
	Harvest
}

impl Interactable {
	pub fn from_str(txt: &str) -> Option<Interactable> {
		match txt {
			"harvest" => Some(Interactable::Harvest),
			_ => None
		}
	}
}
