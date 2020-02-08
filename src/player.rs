
use crate::components::{Player, Visible};
use crate::componentwrapper::{ComponentWrapper, PreEntity};

pub fn make_player(name: &str) -> PreEntity {
	vec![
		ComponentWrapper::Visible(Visible {
			sprite: "player".to_string(),
			height: 1.0
		}),
		ComponentWrapper::Player(Player::new(name.to_string()))
	]
}
