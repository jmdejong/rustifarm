
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use specs::Builder;
use std::str::FromStr;

use crate::{
	PlayerId,
	RoomId,
	ItemId,
	Sprite,
	playerstate::RoomPos,
	components::{
		AttackType,
		Clan,
		Flag,
		Trigger,
		Stat
	},
	parameter::{Parameter},
	fromtoparameter::FromToParameter,
	Timestamp,
	Template,
	Pos,
	Result,
	aerr,
	errors
};


macro_rules! components {
	(post: $($comp: ident ($($paramname: ident : $paramtype: ty, $extraction: expr),*) $creation: expr);*;) => {
		#[derive(Clone)]
		pub enum ComponentWrapper{
			$(
				$comp(crate::components::$comp),
			)*
		}
		impl ComponentWrapper {
			pub fn build<A: Builder>(&self, builder: A ) -> A {
				match self.clone() {
					$(
						Self::$comp(c) => builder.with(c),
					)*
				}
			}
			pub fn load_component(comptype: ComponentType, mut parameters: HashMap<&str, Parameter>) -> Result<Self> {
				#[allow(unused_imports, unreachable_code, unused_braces)]
				match comptype {
					$(
						ComponentType::$comp => Ok(Self::$comp({
							use crate::components::$comp;
							$(
								let $paramname = {
									let param = parameters
										.remove(stringify!($paramname))
										.ok_or(aerr!("required parameter '{}'not found", stringify!($paramname)))?;
									<$paramtype>::from_parameter(&param)
										.ok_or(aerr!("parameter {} is invalid type: {:?} is not of type {}", stringify!($paramname), param, stringify!($paramtype)))?
								};

							)*
							$creation
						})),
					)*
				}
			}
		}
		#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, EnumString, Display)]
		pub enum ComponentType {
			$(
				$comp,
			)*
		}
		impl ComponentType {
			pub fn parameters(&self) -> Vec<&str> {
				match self {
					$(
						Self::$comp => {
							#[allow(unused_mut)]
							let mut h = Vec::new();
							$(
								h.push(stringify!($paramname));
							)*
							h
						},
					)*
				}
			}
		}
		
		use specs::{World, Entity, WorldExt};
		pub fn extract_parameter(typ: ComponentType, parameter: &str, world: &World, ent: Entity) -> Option<Parameter> {
			match typ {
				$(
					
					#[allow(path_statements)]
					ComponentType::$comp => {
						$(
							if parameter == stringify!($paramname) {
								#[allow(unreachable_code, non_snake_case)]
								return Some({
									let components = world.read_component::<crate::components::$comp>();
									#[allow(unused_variables)]
									let $comp = components.get(ent)?;
									#[allow(unused_variables)]
									let extracted: $paramtype = ({$extraction});
									return Some(extracted.to_parameter())
								})
							}
						)*
						None::<Parameter> 
					}
				)*
			}
		}
	};
	// no parameters: make unit struct
	(pre: ($($done: tt)*) $comp: ident; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp () {$comp};) $($tail)*);
	};
	// struct is just parameters
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ty),*);$($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, {$comp.$paramname.clone()}),*) {$comp{$($paramname,)*}};) $($tail)*);
	};
	// full definition minus variable exraction
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ty),*) $creation: expr; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, {None?}),*) $creation;) $($tail)*);
	};
	// full definition
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ty, ($extraction: expr)),*) $creation: expr; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, $extraction),*) $creation;) $($tail)*);
	};
	(pre: ($($done: tt)*)) => {
		components!(post: $($done)*);
	};
	(all: $($all: tt)*) => {components!(pre: () $($all)*);};
}

components!(all: 
	Visible (name: String, sprite: Sprite, height: f64);
	Movable (cooldown: i64);
	Player (name: PlayerId) {Player::new(name)};
	Item (item: ItemId) {Item(item)};
	Inventory () {panic!("inventory from parameters not implemented")};
	Health (health: i64, maxhealth: i64);
	Serialise () {panic!("serialise from parameters not implemented")};
	RoomExit (destination: RoomId, dest_pos: String) {
		RoomExit {
			destination,
			dest_pos: if dest_pos.is_empty() {
					RoomPos::Unknown
				} else {
					RoomPos::Name(dest_pos)
				}
		}
	};
	Trap (damage: i64) {Trap{attack: AttackType::Attack(damage)}};
	Fighter (damage: i64, cooldown: i64) {Fighter{attack: AttackType::Attack(damage), cooldown, range: 1}};
	Healing (delay: i64, health: i64) {Healing{delay, health, next_heal: None}};
	Autofight () {Autofight::default()};
	MonsterAI (move_chance: f64, view_distance: i64, homesickness: f64, home: Option<Pos>);
	Spawner (amount: i64, clan: String, template: Template, radius: i64) {
		Spawner{
			amount: amount as usize,
			clan: Clan{name: clan},
			template: template,
			saturated: false,
			radius
		}
	};
	Clan (name: String);
	Faction (faction: String) {Faction::from_str(faction.as_str()).map_err(|_|aerr!("invalid faction name '{}'", faction))?};
	Interactable (typ: String, arg: Parameter) {
		Interactable::parse_from_parameter(&typ, &arg).ok_or(aerr!("invalid interaction {} {:?}", typ, arg))?
	};
	Loot (loot: Vec<(Template, f64)>);
	Timer (trigger: Trigger, delay: i64, spread: f64, target_time: Option<Timestamp>);
	TimeOffset (dtime: i64);
	Flags (flags: Vec<String>) {
		Flags(
			flags
				.iter()
				.map(|s| Flag::from_str(s).ok())
				.collect::<Option<HashSet<Flag>>>().ok_or(aerr!("invalid flag name"))?
		)
	};
	Ear () {Ear::default()};
	Build (obj: Template);
	Whitelist (allowed: HashMap<String, HashSet<PlayerId>>);
	Minable (trigger: String, total: i64) {
		Minable {
			trigger: Trigger::from_str(&trigger).map_err(|_|aerr!("invalid trigger name {}", trigger))?,
			progress: 0,
			total
		}
	};
	Removed;
	LootHolder () {panic!("LootHolder from parameters not implemented")};
	OnSpawn (trigger: String) {
		OnSpawn {
			trigger: Trigger::from_str(&trigger).map_err(|_|aerr!("invalid trigger name {}", trigger))?
		}
	};
	Substitute (into: Template);
	TriggerBox (triggers: Vec<String>) {
		TriggerBox {
			messages: triggers.iter().map(|s|
					Trigger::from_str(s).map_err(|_|aerr!("invalid trigger name {}", s))
				).collect::<std::result::Result<Vec<Trigger>, std::boxed::Box<errors::AError>>>()?
		}
	};
	Stats (skills: HashMap<Stat, i64>);
	Requirements (required_flags: HashSet<Flag>, blocking_flags: HashSet<Flag>);
	Description (description: String);
);


pub type PreEntity = Vec<ComponentWrapper>;




