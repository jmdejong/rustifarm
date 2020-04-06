
use specs::{
	HashMapStorage,
	Component,
};


#[derive(Debug, Clone)]
pub struct Sound {
	pub source: Option<String>,
	pub text: String
}

impl Sound {
	pub fn as_message(self) -> (Option<String>, String) {
		(None, format!("{}: {}", self.source.unwrap_or("".to_string()), self.text))
	}
}

#[derive(Component, Debug, Clone, Default)]
#[storage(HashMapStorage)]
pub struct Ear{
	pub sounds: Vec<Sound>
}
