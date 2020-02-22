
use serde_json::{Value, json};
use crate::template::Template;
use crate::components::item::ItemAction;

#[derive(Debug, PartialEq, Clone)]
pub enum Parameter {
	String(String),
	Int(i64),
// 	Pos(Pos),
	Float(f64),
	Template(Template),
	Action(ItemAction)
}

impl Parameter {
	
	#[allow(dead_code)]
	pub fn string(string: &str) -> Self {
		Self::String(string.to_string())
	}
	
	pub fn from_typed_json(typ: ParameterType, val: &Value) -> Option<Parameter>{
		match typ {
			ParameterType::String => Some(Self::String(val.as_str()?.to_string())),
			ParameterType::Int => Some(Self::Int(val.as_i64()?)),
			ParameterType::Float => Some(Self::Float(val.as_f64()?)),
			ParameterType::Template => Some(Self::Template(Template::from_json(val).ok()?)),
			ParameterType::Action => Some(Self::Action(ItemAction::from_json(val)?))
		}
	}
	
	pub fn paramtype(&self) -> ParameterType {
		match self {
			Self::String(_) => ParameterType::String,
			Self::Int(_) => ParameterType::Int,
			Self::Float(_) => ParameterType::Float,
			Self::Template(_) => ParameterType::Template,
			Self::Action(_) => ParameterType::Action
		}
	}
	
	pub fn guess_from_json(val: &Value) -> Option<Parameter> {
		let typ = 
			if val.is_string() {
				ParameterType::String
			} else if val.is_u64() || val.is_i64() {
				ParameterType::Int
			} else if val.is_f64() {
				ParameterType::Float
			} else if val.is_object(){
				ParameterType::Template
			} else {
				println!("{:?}", val);
				return None
			};
		Self::from_typed_json(typ, val)
	}
	
	pub fn to_json(&self) -> Value {
		match self {
			Self::String(s) => json!(s),
			Self::Int(i) => json!(i),
			Self::Float(f) => json!(f),
			Self::Template(t) => t.to_json(),
			Self::Action(a) => a.to_json()
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
	String,
	Int,
	Float,
	Template,
	Action
}

impl ParameterType {
	
	pub fn from_str(typename: &str) -> Option<Self>{
		match typename {
			"string" => Some(Self::String),
			"int" => Some(Self::Int),
			"float" => Some(Self::Float),
			"template" => Some(Self::Template),
			"action" => Some(Self::Action),
			_ => None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	macro_rules! gfj { // guess from json
		($j:expr) => {Parameter::guess_from_json(&json!($j)).unwrap()}
	}
	
	#[test]
	fn can_guess_json() {
		Parameter::guess_from_json(&json!(3)).unwrap();
	}
	
	#[test]
	fn guess_json() {
		assert_eq!(gfj!("charles"), Parameter::string("charles"));
		assert_eq!(gfj!("1"), Parameter::string("1"));
		assert_eq!(gfj!(""), Parameter::string(""));
		assert_eq!(gfj!(3), Parameter::Int(3));
		assert_eq!(gfj!(-3), Parameter::Int(-3));
		assert_eq!(gfj!(0), Parameter::Int(0));
		assert_eq!(gfj!(-0), Parameter::Int(0));
		assert_eq!(gfj!(3.5), Parameter::Float(3.5));
		assert_eq!(gfj!(3.0), Parameter::Float(3.0));
		assert_eq!(gfj!(-3.0), Parameter::Float(-3.0));
		assert_eq!(gfj!(0.0), Parameter::Float(0.0));
		assert_eq!(gfj!(-0.0), Parameter::Float(0.0));
	}
	
	#[test]
	fn guess_json_none() {
		assert!(Parameter::guess_from_json(&json!([2, 5])).is_none());
		assert!(Parameter::guess_from_json(&json!(true)).is_none());
		assert!(Parameter::guess_from_json(&json!({"hello": "world"})).is_none());
	}
}
