
use std::collections::{HashMap, HashSet};
use specs::Builder;
use rand::Rng;

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
		Trigger
	},
	parameter::{Parameter, ParameterType},
	Timestamp,
	Template,
	Result,
	aerr
};


macro_rules! components {
	(post: $($comp: ident ($($paramname: ident : $paramtype: ident, $extraction: expr),*) $creation: expr);*;) => {
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
				#[allow(unused_imports, unreachable_code)]
				match comptype {
					$(
						ComponentType::$comp => Ok(Self::$comp({
							use crate::components::$comp;
							$(
								let $paramname = match parameters.remove(stringify!($paramname))
										.ok_or(aerr!("required parameter '{}'not found", stringify!($paramname)))? {
									Parameter::$paramtype(p) => p,
									x => Err(aerr!("parameter type mismatch for parameter {}: {} {:?}", stringify!($paramname), stringify!($paramtype), x))?
								};
							)*
							$creation
						})),
					)*
				}
			}
		}
		#[derive(Debug, PartialEq, Eq, Clone, Copy)]
		pub enum ComponentType {
			$(
				$comp,
			)*
		}
		impl ComponentType {
			pub fn from_str(typename: &str) -> Option<ComponentType>{
				match typename {
					$(
						stringify!($comp) => Some(Self::$comp),
					)*
					_ => None
				}
			}
			pub fn parameters(&self) -> HashMap<&str, ParameterType> {
				match self {
					$(
						Self::$comp => {
							#[allow(unused_mut)]
							let mut h = HashMap::new();
							$(
								h.insert(stringify!($paramname), ParameterType::$paramtype);
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
								return Some(Parameter::$paramtype({
									let components = world.read_component::<crate::components::$comp>();
									#[allow(unused_variables)]
									let $comp = components.get(ent)?;
									$extraction
								}))
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
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ident),*);$($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, {$comp.$paramname.clone()}),*) {$comp{$($paramname,)*}};) $($tail)*);
	};
	// full definition minus variable exraction
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ident),*) $creation: expr; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, {None?}),*) $creation;) $($tail)*);
	};
	// full definition
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ident ($extraction: expr)),*) $creation: expr; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, $extraction),*) $creation;) $($tail)*);
	};
	(pre: ($($done: tt)*)) => {
		components!(post: $($done)*);
	};
	($($all: tt)*) => {components!(pre: () $($all)*);};
}

components!(
	Visible (name: String, sprite: String, height: Float) {
		Visible {
			sprite: Sprite{name: sprite},
			height,
			name
		}
	};
	Movable (cooldown: Int);
	Player (name: String) {Player::new(PlayerId{name})};
	Item (item: String) {Item(ItemId(item))};
	Inventory () {panic!("inventory from parameters not implemented")};
	Health (health: Int, maxhealth: Int);
	Serialise () {panic!("serialise from parameters not implemented")};
	RoomExit (destination: String, dest_pos: String) {
		RoomExit {
			destination: RoomId::from_str(&destination),
			dest_pos: if dest_pos.is_empty() {
					RoomPos::Unknown
				} else {
					RoomPos::Name(dest_pos)
				}
		}
	};
	Trap (damage: Int) {Trap{attack: AttackType::Attack(damage)}};
	Fighter (damage: Int, cooldown: Int) {Fighter{attack: AttackType::Attack(damage), cooldown, range: 1}};
	Healing (delay: Int, health: Int) {Healing{delay, health, next_heal: None}};
	Autofight () {Autofight::default()};
	MonsterAI (move_chance: Float, homesickness: Float, view_distance: Int);
	Spawner (amount: Int, clan: String, template: Template) {
		Spawner{
			amount: amount as usize,
			clan: Clan{name:
				if clan == "" {
					format!("$random({})", rand::thread_rng().gen::<u32>())
				} else {
					clan
				}
			},
			template: template.unsaved(),
			saturated: false
		}
	};
	Clan (name: String);
	Home (home: Pos);
	Faction (faction: String) {Faction::from_str(faction.as_str()).ok_or(aerr!("invalid faction name"))?};
	Interactable (action: Interaction) {action};
	Loot (loot: List) {
		Loot { loot:
			loot
			.iter()
			.map(|param| {match param {
				Parameter::Template(template) => Ok((template.clone(), 1.0)),
				Parameter::List(l) => {
					if l.len() == 2 {
						if let (Parameter::Template(template), Parameter::Float(chance)) = (l[0].clone(), l[1].clone()) {
							return Ok((template.clone(), chance))
						}
					}
					Err(aerr!("loot list elements as list must only contain a template and a float: {:?}", l))?
				},
				_ => Err(aerr!("loot list elements must be a template or a list: {:?}", param))?
			}})
			.collect::<Result<Vec<(Template, f64)>>>()?
		}
	};
	Timer (
			trigger: String (panic!("can't turn trigger to string")),
			delay: Int (Timer.delay),
			spread: Float (Timer.spread),
			target_time: Int ({
				if let Some(time) = Timer.target_time {
					time.0
				} else {
					0
				}
			})
		)
		Timer {
			trigger: Trigger::from_str(&trigger).ok_or(aerr!("invalid trigger name {}", trigger))?,
			delay,
			spread,
			target_time: if target_time == -1 { None } else { Some(Timestamp(target_time)) }
			// please forgive me for using -1 as null
		};
	Equipment () {panic!("equipment from parameters not implemented")};
	TimeOffset (dtime: Int);
	Flags (flags: List) {
		Flags(
			flags
				.iter()
				.map(|param| {
					if let Parameter::String(f) = param {
						Flag::from_str(f)
					} else {
						None
					}
				})
				.collect::<Option<HashSet<Flag>>>().ok_or(aerr!("invalid flag name"))?
		)
	};
	Ear () {Ear::default()};
	Build (obj: Template);
);


pub type PreEntity = Vec<ComponentWrapper>;




