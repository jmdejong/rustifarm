
use serde_json::{Value, json};
use serde::{de, Serialize, Deserialize, Serializer, Deserializer};
use crate::{
	Template,
	Pos,
	PResult,
	perr
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
			pub fn from_typed_json(typ: ParameterType, val: &Value) -> PResult<Parameter>{
				match typ {
					$(
						ParameterType::$name => Ok(Self::$name({
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
	String (String) string, v (v.as_str().ok_or(perr!("{:?} not a string", v))?.to_string()) (json!(v));
	Int (i64) int, v (v.as_i64().ok_or(perr!("{:?} not an int", v))?) (json!(v));
	Pos (Pos) pos, v (Pos::deserialize(v).map_err(|e| perr!("{:?} not a pos {}", v, e))?) (json!(v));
	Float (f64) float, v (v.as_f64().ok_or(perr!("{:?} not an float", v))?) (json!(v));
	Template (Template) template, v (Template::deserialize(v).map_err(|e| perr!("template json error {:?}", e))?) (json!(["template", v]));
	Bool (bool) bool, v (v.as_bool().ok_or(perr!("{:?} not a bool", v))?) (json!(v));
	List (Vec<Parameter>) list, v 
		({
			v
				.as_array().ok_or(perr!("{:?} not an array", v))?
				.iter()
				.map(|item| Parameter::guess_from_json(item))
				.collect::<PResult<Vec<Parameter>>>()?
		})
		(json!(["list", v.iter().map(Parameter::to_json).collect::<Vec<Value>>()]));
);


impl Parameter {
	#[allow(dead_code)]
	pub fn string(string: &str) -> Self {
		Self::String(string.to_string())
	}
	
	pub fn guess_from_json(val: &Value) -> PResult<Parameter> {
		if let Some(arr) = val.as_array() {
			if arr.len() == 2 && arr[0].is_string() {
				let typestr = arr[0].as_str().unwrap();
				let typ = ParameterType::from_str(typestr).ok_or(perr!("invalid parameter type {}", typestr))?;
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
				return Err(perr!("can't guess the type of parameter {:?}", val));
			};
		Self::from_typed_json(typ, val)
	}
}


impl Serialize for Parameter {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		self.to_json().serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for Parameter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		Self::guess_from_json(&Value::deserialize(deserializer)?).map_err(|e| de::Error::custom(e.text))
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
		assert!(Parameter::guess_from_json(&json!([2, 5])).is_err());
		assert!(Parameter::guess_from_json(&json!({"hello": "world"})).is_err());
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
