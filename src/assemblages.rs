
use std::collections::HashMap;
use rand::Rng;
use serde_json::Value;
use specs::{
	Builder,
	EntityBuilder
};

use super::components::{Visible, Blocking, Played};

macro_rules! assemblage {
	($name:ident { $($arg:ident : $argt:ident ),* } ; $( $comp:expr ),* ) => {
		#[derive(Debug, Clone, Default)]
		pub struct $name {$(
			pub $arg : $argt
		)* }
		impl Assemblage for $name {
			fn build<'a>(&self, mut builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
				$(
					let $arg = &self.$arg;
				)*
				$(
					builder = builder.with($comp);
				)*
				builder
			}
			
			#[allow(unused_variables, unused_mut)]
			fn init_from_json(&mut self, mut args: Vec<Value>, kwargs: HashMap<&str, Value>) {
				$(
					if args.len() > 0 {
						let val = args.remove(0);
						if let Some(actual_val) = unpack_json!($argt, val) {
							self.$arg = actual_val;
						}
					}
				)*
				$(
					if let Some(val) = kwargs.get(stringify!($arg)) {
						if let Some(actual_val) = unpack_json!($argt, val) {
							self.$arg = actual_val;
						}
					}
				)*
			}
		}
	}
}

macro_rules! unpack_json {
	(String, $val: ident) => {
		if let Some(txt) = $val.as_str(){
			Some(txt.to_string())
		} else {
			None
		}
	}
}


pub trait Assemblage: Send + Sync {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>;
	fn init_from_json(&mut self, args: Vec<Value>, kwargs: HashMap<&str, Value>);
}



assemblage!(Wall {}; Visible{sprite: "wall".to_string(), height: 2.0}, Blocking);

assemblage!(Grass { sprite : String}; Visible{sprite: sprite.to_string(), height: 0.1});

impl Grass {
	pub fn new() -> Grass {
		Grass {
			sprite: ["grass1", "grass2", "grass3", "grass1", "grass2", "grass3", "ground"][rand::thread_rng().gen_range(0,7)].to_string()
		}
	}
}


assemblage!(Player {name: String}; Visible{sprite: "player".to_string(), height: 1.0}, Played::new(name.to_string()));

impl Player {	
	pub fn new(name: &str) -> Player {
		Player { name: name.to_string()}
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	use super::super::hashmap;
	#[test]
	fn test_assemblage_from_json() {
		let mut p = Player::new("Joe");
		assert_eq!(p.name, "Joe");
		p.init_from_json(vec![json!("Bob"), json!("Mike")], HashMap::new());
		assert_eq!(p.name, "Bob");
		p.init_from_json(vec![], hashmap!("sprite" => json!("stone")));
		assert_eq!(p.name, "Bob");
		p.init_from_json(vec![], hashmap!("name" => json!("Teddy")));
		assert_eq!(p.name, "Teddy");
		p.init_from_json(vec![json!("Bill")], hashmap!("name" => json!("Stan")));
		assert_eq!(p.name, "Stan");
	}
}


