
use specs::Entity;

#[derive(Debug, Clone, Default)]
pub struct Attack {
	pub damage: i64,
	pub attacker: Option<Entity>
}

impl Attack {
	pub fn new(damage: i64) -> Self {
		Self {
			damage,
			attacker: None
		}
	}
}
