

use rand::Rng;
use specs::{
	Builder,
	EntityBuilder
};

use super::components::{Visible, Blocking, Played};

macro_rules! assemblage {
	($name:ident { $($arg:ident : $argt:ident ),* } ; $( $comp:expr ),* ) => {
		#[derive(Debug, Clone)]
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
		}
		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}
	}
}


pub trait Assemblage: Send + Sync {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>;
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


assemblage!(Player {name: String}; Visible{sprite: "player".to_string(), height: 1.0}, Played{name: name.to_string()});

impl Player {	
	pub fn new(name: &str) -> Player {
		Player { name: name.to_string()}
	}
}
