
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Parameter {
	String(String),
	Int(i64),
// 	Pos(Pos),
	Float(f64)
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
			ParameterType::Float => Some(Self::Float(val.as_f64()?))
		}
	}
	
	pub fn paramtype(&self) -> ParameterType {
		match self {
			Self::String(_) => ParameterType::String,
			Self::Int(_) => ParameterType::Int,
			Self::Float(_) => ParameterType::Float
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
			} else {
				println!("{:?}", val);
				return None
			};
		Self::from_typed_json(typ, val)
	}
	
	pub fn as_str(&self) -> Option<&str> {
		if let Self::String(str) = self {
			Some(str)
		} else {
			None
		}
	}
	
	pub fn as_string(&self) -> Option<String> {
		Some(self.as_str()?.to_string())
	}
	
	#[allow(dead_code)]
	pub fn as_i64(&self) -> Option<i64> {
		if let Self::Int(num) = self {
			Some(*num)
		} else {
			None
		}
	}
	
	pub fn as_f64(&self) -> Option<f64> {
		if let Self::Float(num) = self {
			Some(*num)
		} else {
			None
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
	String,
// 	Pos,
	Int,
	Float
}

impl ParameterType {
	
	pub fn from_str(typename: &str) -> Option<Self>{
		match typename {
			"string" => Some(Self::String),
			"int" => Some(Self::Int),
			"float" => Some(Self::Float),
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
