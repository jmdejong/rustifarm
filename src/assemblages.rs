

use rand::Rng;
use specs::{
	Builder,
	EntityBuilder
};

use super::components::{Visible};




pub trait Assemblage {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>;
}



pub struct Wall;

impl Assemblage for Wall {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
		builder.with(Visible{sprite: "wall".to_string(), height: 2.0})
	}
}

pub struct Grass {
	sprite: String
}

impl Grass {
	pub fn new() -> Grass {
		Grass {
			sprite: ["grass1", "grass2", "grass3", "grass1", "grass2", "grass3", "ground"][rand::thread_rng().gen_range(0,7)].to_string()
		}
	}
}

impl Assemblage for Grass {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
		builder.with(Visible{sprite: self.sprite.to_string(), height: 0.1})
	}
}


pub struct Player {
	name: String
}

impl Player {	
	pub fn new(name: &str) -> Player {
		Player { name: name.to_string()}
	}
}

impl Assemblage for Player {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
		builder.with(Visible{sprite: "player".to_string(), height: 1.0})
	}
}
