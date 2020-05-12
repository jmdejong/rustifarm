
use serde_json::{Value, json};
use crate::{
	Template,
	components::interactable::Interactable,
	Pos
};



macro_rules! parameters {
	($($name: ident ($typ: ty) $stringname: ident, $v: ident ($fromjson: expr) ($tojson: expr));*;) => {
		#[derive(Debug, PartialEq, Clone)]
		pub enum Parameter {
			$(
				$name($typ),
			)*
		}
		impl Parameter {
			pub fn from_typed_json(typ: ParameterType, val: &Value) -> Option<Parameter>{
				match typ {
					$(
						ParameterType::$name => Some(Self::$name({
							let $v = val;
							$fromjson
						})),
					)*
				}
			}
			pub fn paramtype(&self) -> ParameterType {
				match self {
					$(
						Self::$name(_) => ParameterType::$name,
					)*
				}
			}
			pub fn to_json(&self) -> Value {
				match self {
					$(
						Self::$name($v) => $tojson,
					)*
				}
			}
		}

		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		pub enum ParameterType {
			$(
				$name,
			)*
		}
		impl ParameterType {
			pub fn from_str(typename: &str) -> Option<Self>{
				match typename {
					$(
						stringify!($stringname) => Some(Self::$name),
					)*
					_ => None
				}
			}
		}
	}
}

parameters!(
	String (String) string, v (v.as_str()?.to_string()) (json!(v));
	Int (i64) int, v (v.as_i64()?) (json!(v));
	Pos (Pos) pos, v (Pos::from_json(v)?) (json!(v));
	Float (f64) float, v (v.as_f64()?) (json!(v));
	Template (Template) template, v (Template::from_json(v).ok()?) (json!(["template", v.to_json()]));
	Interaction (Interactable) interaction, _v (Interactable::from_json(_v)?) (panic!("interactions can't be serialized"));
	Bool (bool) bool, v (v.as_bool()?) (json!(v));
	List (Vec<Parameter>) list, v 
		({
			v
				.as_array()?
				.iter()
				.map(|item| Parameter::guess_from_json(item))
				.collect::<Option<Vec<Parameter>>>()?
		})
		(json!(["list", v.iter().map(Parameter::to_json).collect::<Vec<Value>>()]));
);


impl Parameter {
	#[allow(dead_code)]
	pub fn string(string: &str) -> Self {
		Self::String(string.to_string())
	}
	
	pub fn guess_from_json(val: &Value) -> Option<Parameter> {
		if let Some(arr) = val.as_array() {
			if arr.len() == 2 && arr[0].is_string() {
				let typestr = arr[0].as_str().unwrap();
				let typ = ParameterType::from_str(typestr)?;
				return Self::from_typed_json(typ, &arr[1]);
			}
		}
		let typ = 
			if val.is_string() {
				ParameterType::String
			} else if val.is_u64() || val.is_i64() {
				ParameterType::Int
			} else if val.is_f64() {
				ParameterType::Float
			} else if val.is_boolean(){
				ParameterType::Bool
			} else if val.is_object(){
				ParameterType::Template
			} else {
				return None
			};
		Self::from_typed_json(typ, val)
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	macro_rules! gfj { // guess from json
		($($j:tt)*) => {Parameter::guess_from_json(&json!($($j)*)).unwrap()}
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
		assert_eq!(gfj!(true), Parameter::Bool(true));
		
		assert_eq!(gfj!(["int", 3]), Parameter::Int(3));
	}
	
	#[test]
	fn guess_json_none() {
		assert!(Parameter::guess_from_json(&json!([2, 5])).is_none());
		assert!(Parameter::guess_from_json(&json!({"hello": "world"})).is_none());
	}
	
	#[test]
	fn parse_list() {
		assert_eq!(
				gfj!(["list", [5, 3, 1, 2]]),
			Parameter::List(vec![
				Parameter::Int(5),
				Parameter::Int(3),
				Parameter::Int(1),
				Parameter::Int(2)
			])
		);
		assert_eq!(
				gfj!(["list", [5, 3.0, "Hello", true]]),
			Parameter::List(vec![
				Parameter::Int(5),
				Parameter::Float(3.0),
				Parameter::string("Hello"),
				Parameter::Bool(true)
			])
		);
	}
}
