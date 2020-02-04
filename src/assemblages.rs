
use rand::Rng;
use super::components::{Visible, Blocking, Played};
use super::assemblage;
use super::assemblage::Assemblage;

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
		p.init_from_json(vec![json!("Bob"), json!("Mike")], hashmap!());
		assert_eq!(p.name, "Bob");
		p.init_from_json(vec![], hashmap!("sprite".to_string() => json!("stone")));
		assert_eq!(p.name, "Bob");
		p.init_from_json(vec![], hashmap!("name".to_string() => json!("Teddy")));
		assert_eq!(p.name, "Teddy");
		p.init_from_json(vec![json!("Bill")], hashmap!("name".to_string() => json!("Stan")));
		assert_eq!(p.name, "Stan");
	}
}


