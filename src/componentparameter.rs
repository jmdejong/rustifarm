
use std::collections::HashMap;
use serde_json::Value;
use crate::parameter::{Parameter, ParameterType};

#[derive(Debug, PartialEq)]
pub enum ComponentParameter {
	Constant(Parameter),
	Argument(String)
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
				_ => Err("unknown compparam type")
			}
		}
	}
	
	pub fn get_type(&self, arguments: &Vec<(String, ParameterType, Option<Parameter>)>) -> Result<ParameterType, &'static str>{
		Ok(match self {
			Self::Constant(param) => param.paramtype(),
			Self::Argument(argname) => arguments.iter().find(|(n, _t, _d)| n == argname).ok_or("unknown argument name")?.1
		})
	}
}
