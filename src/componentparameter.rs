
use std::collections::HashMap;
use rand::Rng;
use serde_json::Value;
use crate::parameter::{Parameter, ParameterType};

#[derive(Debug, PartialEq)]
pub enum ComponentParameter {
	Constant(Parameter),
	Argument(String),
	Random(Vec<ComponentParameter>)
}

impl ComponentParameter {
	pub fn evaluate(&self, arguments: &HashMap<&str, Parameter>) -> Option<Parameter> {
		match self {
			Self::Constant(val) => {
				Some(val.clone())
			},
			Self::Argument(argname) => {
				Some(arguments.get(argname.as_str())?.clone())
			}
			Self::Random(options) => {
				let r = rand::thread_rng().gen_range(0, options.len());
				options[r].evaluate(arguments)
			}
		}
	}
	
	pub fn from_json(value: &Value) -> Result<Self, &'static str> {
		let paramvalue = value.get(1).ok_or("index 0 not in component parameter")?;
		let typename = value.get(0).ok_or("index 0 not in component parameter")?.as_str().ok_or("compparam type not a string")?;
		if let Some(paramtype) = ParameterType::from_str(typename) {
			Ok(Self::Constant(Parameter::from_typed_json(paramtype, paramvalue).ok_or("failed to parse parameter constant")?))
		} else {
			match typename {
				"A" | "arg" => {
					let argname = paramvalue.as_str().ok_or("argument parameter not a string")?.to_string();
					Ok(Self::Argument(argname))
				},
				"random" => {
					let optionvalues = paramvalue.as_array().ok_or("random argument not a a string")?;
					let mut options = Vec::new();
					for option in optionvalues {
						options.push(Self::from_json(option)?)
					}
					Ok(Self::Random(options))
				},
				_ => Err("unknown compparam type")
			}
		}
	}
	
	pub fn get_type(&self, arguments: &Vec<(String, ParameterType, Option<Parameter>)>) -> Result<ParameterType, &'static str>{
		Ok(match self {
			Self::Constant(param) => param.paramtype(),
			Self::Argument(argname) => arguments.iter().find(|(n, _t, _d)| n == argname).ok_or("unknown argument name")?.1,
			Self::Random(options) => {
				let typ: ParameterType = options.get(0).ok_or("random has no options")?.get_type(arguments)?;
				for param in options {
					if param.get_type(arguments)? != typ {
						return Err("inconsistent parameter types");
					}
				}
				typ
			}
		})
	}
}