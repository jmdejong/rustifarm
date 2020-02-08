
use std::collections::HashMap;
use specs::{Builder, EntityBuilder};

use crate::components::{Visible, Blocking, Player, Floor};
use crate::hashmap;
use crate::parameter::{Parameter, ParameterType};



macro_rules! components {
	($($comp: ident [$($paramname: ident : $paramtype: ident),*] $paramlist: ident {$creation: expr});*) => {
	
	#[derive(Clone)]
	pub enum ComponentWrapper{
		$(
			$comp($comp),
		)*
	}

	impl ComponentWrapper {

		pub fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
			match self.clone() {
				$(
					Self::$comp(c) => builder.with(c),
				)*
			}
		}
		
		pub fn load_component(comptype: ComponentType, mut parameters_: HashMap<&str, Parameter>) -> Option<Self> {
			
			match comptype {
				$(
					ComponentType::$comp => Some(Self::$comp({
						let mut $paramlist = parameters_;
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
}}

components!(
	Visible [sprite: String, height: Float] parameters {
		Visible {
			sprite: parameters.remove("sprite")?.as_string()?,
			height: parameters.remove("height")?.as_f64()?
		}
	};
	Blocking [] p {Blocking};
	Floor [] p {Floor};
	Player [name: String] parameters {Player::new(parameters.remove("name")?.as_string()?)}
);







