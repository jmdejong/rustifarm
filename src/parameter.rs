
use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use crate::{
	Template,
};



macro_rules! parameters {
	{$($name: ident $typ: ty);*;} => {
		#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
		#[serde(untagged)]
		pub enum Parameter {
			$(
				$name($typ),
			)*
		}
		impl Parameter {
			pub fn paramtype(&self) -> ParameterType {
				match self {
					$(
						Self::$name(_) => ParameterType::$name,
					)*
				}
			}
		}

		#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display)]
		#[serde(rename_all = "lowercase")]
		#[strum(serialize_all = "lowercase")]
		pub enum ParameterType {
			$(
				$name,
			)*
		}
	}
}

parameters!{
	String String;
	Int i64;
	Float f64;
	Template Template;
	Bool bool;
	List Vec<Parameter>;
}


impl Parameter {
	#[allow(dead_code)]
	pub fn string(string: &str) -> Self {
		Self::String(string.to_string())
	}
	
}


#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	macro_rules! gfj { // guess from json
		($($j:tt)*) => {Parameter::deserialize(&json!($($j)*)).unwrap()}
	}
	
	#[test]
	fn can_guess_json() {
		Parameter::deserialize(&json!(3)).unwrap();
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
		assert_eq!(gfj!(true), Parameter::Bool(true));
		
		assert_eq!(gfj!(["int", 3]), Parameter::List(vec![Parameter::string("int"), Parameter::Int(3)]));
		assert_eq!(gfj!([2, 5]), Parameter::List(vec![Parameter::Int(2), Parameter::Int(5)]));
	}
	
	#[test]
	fn guess_json_none() {
		assert!(Parameter::deserialize(&json!({"hello": "world"})).is_err());
	}
	
	#[test]
	fn parse_list() {
		assert_eq!(
				gfj!([5, 3, 1, 2]),
			Parameter::List(vec![
				Parameter::Int(5),
				Parameter::Int(3),
				Parameter::Int(1),
				Parameter::Int(2)
			])
		);
		assert_eq!(
				gfj!([5, 3.0, "Hello", true]),
			Parameter::List(vec![
				Parameter::Int(5),
				Parameter::Float(3.0),
				Parameter::string("Hello"),
				Parameter::Bool(true)
			])
		);
	}
}
