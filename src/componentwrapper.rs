
use std::collections::HashMap;
use specs::Builder;

use crate::{
	PlayerId,
	RoomId,
	Sprite,
	playerstate::RoomPos,
	attack::Attack,
	components::{Visible, Blocking, Player, Floor, Item, Inventory, Health, Serialise, RoomExit, Trap, Fighter},
	parameter::{Parameter, ParameterType}
};



macro_rules! components {
	($($comp: ident ($($paramname: ident : $paramtype: ident),*) {$creation: expr});*;) => {
	
		#[derive(Clone)]
		pub enum ComponentWrapper{
			$(
				$comp($comp),
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
			
			pub fn load_component(comptype: ComponentType, mut parameters: HashMap<&str, Parameter>) -> Option<Self> {
				
				match comptype {
					$(
						
						ComponentType::$comp => Some(Self::$comp({
							$(
								let $paramname = match parameters.remove(stringify!($paramname))? {
									Parameter::$paramtype(p) => p,
									_ => {return None}
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
	}
}

components!(
	Visible (name: String, sprite: String, height: Float) {
		Visible {
			sprite: Sprite{name: sprite},
			height,
			name
		}
	};
	Blocking () {Blocking};
	Floor () {Floor};
	Player (name: String) {Player::new(PlayerId{name})};
	Item (ent: Template, name: String, action: Action) {Item{ent, name, action}};
	Inventory (capacity: Int) {Inventory{items: Vec::new(), capacity: capacity as usize}};
	Health (health: Int, maxhealth: Int) {Health{health, maxhealth}};
	Serialise (template: Template) {Serialise{template}};
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
	Trap (damage: Int) {Trap{attack: Attack::new(damage)}};
	Fighter (damage: Int) {Fighter{attack: Attack::new(damage)}};
);


pub type PreEntity = Vec<ComponentWrapper>;




