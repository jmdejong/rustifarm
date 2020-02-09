
use std::collections::HashMap;
use specs::{Builder, world::LazyBuilder};

use crate::components::{Visible, Blocking, Player, Floor};
use crate::parameter::{Parameter, ParameterType};



macro_rules! components {
	($($comp: ident ($($paramname: ident : $paramtype: ident),*) {$creation: expr});*) => {
	
		#[derive(Clone)]
		pub enum ComponentWrapper{
			$(
				$comp($comp),
			)*
		}

		impl ComponentWrapper {

			pub fn build<'a>(&self, builder: LazyBuilder<'a>) -> LazyBuilder<'a> {
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
	Visible (sprite: String, height: Float) {
		Visible {
			sprite,
			height
		}
	};
	Blocking () {Blocking};
	Floor () {Floor};
	Player (name: String) {Player::new(name.to_string())}
);

pub type PreEntity = Vec<ComponentWrapper>;






