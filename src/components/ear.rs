
use specs::{
	HashMapStorage,
	Component,
	Entity,
	WriteStorage
};


#[derive(Debug, Clone)]
pub enum HealthNotification {
	Attack,
	Damage,
	Heal
}

use HealthNotification::*;

#[derive(Debug, Clone)]
pub enum Notification {
	Sound{
		source: Option<String>,
		text: String
	},
	Health {
		actor: String,
		target: String,
		amount: i64,
		typ: HealthNotification
	},
	Kill {
		actor: String,
		target: String
	},
	Die {
		actor: String,
		target: String
	},
	Options {
		description: String,
		options: Vec<(String, String)>
	}
}

use Notification::*;


impl Notification {
	
	
	pub fn type_name(&self) -> String {
		(match self {
			Sound{source: _, text: _} => "sound",
			Health{actor: _, target: _, amount: _, typ} => match typ {
				Attack => "attack",
				Damage => "damage",
				Heal => "heal"
			},
			Kill{actor: _, target: _} => "kill",
			Die{actor: _, target: _} => "die",
			Options{description: _, options: _} => "options"
		}).to_string()
	}
	
	pub fn as_message(&self) -> (String, String) {
		let body = match self {
			Sound{source, text} => {
				if let Some(name) = &source {
					format!("{}: {}", name, &text)
				} else {
					text.clone()
				}
			}
			Health{actor, target, amount, typ} => {
				match typ {
					Attack | Damage => format!("{} attacks {} for {} damage", actor, target, amount),
					Heal => format!("{} heals {} for {} health", actor, target, amount)
				}
			},
			Kill{actor, target} => {
				format!("{} kills {}", actor, target)
			},
			Die{actor, target} => {
				format!("{} was killed by {}", target, actor)
			},
			Options{description, options} => {
				format!("{}. Options: {}", description, options.iter().map(|(command, desc)| format!("'{}': {};", command, desc)).collect::<Vec<String>>().join(" "))
			}
		};
		(self.type_name(), body)
	}
}

#[derive(Component, Debug, Clone, Default)]
#[storage(HashMapStorage)]
pub struct Ear{
	pub sounds: Vec<Notification>
}

pub fn say(ears: &mut WriteStorage<Ear>, ent: Entity, msg: Notification){
	if let Some(ear) = ears.get_mut(ent) {
		ear.sounds.push(msg);
	}
}

