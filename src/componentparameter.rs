
use std::collections::HashMap;
use rand::Rng;
use serde_json::Value;
use crate::{
	parameter::{Parameter, ParameterType},
	Template,
	Result,
	aerr,
	PResult,
	perr
};

const MAX_NESTING: usize = 5;


#[derive(Debug, PartialEq, Clone)]
pub enum ComponentParameter {
	Constant(Parameter),
	Argument(String),
	Random(Vec<ComponentParameter>),
	Concat(Vec<ComponentParameter>),
	If(Box<ComponentParameter>, Box<ComponentParameter>, Box<ComponentParameter>),
	TemplateSelf,
	TemplateName
}

impl ComponentParameter {

	pub fn evaluate(&self, arguments: &HashMap<&str, Parameter>, template: &Template) -> Option<Parameter> {
		self.evaluate_(arguments, template, 0)
	}
	
	fn evaluate_(&self, arguments: &HashMap<&str, Parameter>, template: &Template, nesting: usize) -> Option<Parameter> {
		if nesting > MAX_NESTING {
			return None;
		}
		match self {
			Self::Constant(val) => {
				Some(val.clone())
			}
			Self::Argument(argname) => {
				Some(arguments.get(argname.as_str())?.clone())
			}
			Self::Random(options) => {
				let r = rand::thread_rng().gen_range(0, options.len());
				options[r].evaluate_(arguments, template, nesting + 1)
			}
			Self::Concat(options) => {
				let mut string = String::new();
				for option in options {
					if let Parameter::String(s) = option.evaluate_(arguments, template, nesting+1)? {
						string.push_str(&s);
					} else {
						return None;
					}
				}
				Some(Parameter::String(string))
			}
			Self::If(condition, thenval, elseval) => {
				if let Parameter::Bool(b) = condition.evaluate_(arguments, template, nesting+1)? {
					if b {
						thenval.evaluate_(arguments, template, nesting+1)
					} else {
						elseval.evaluate_(arguments, template, nesting+1)
					}
				} else {
					None
				}
			}
			Self::TemplateSelf => Some(Parameter::Template(template.clone())),
			Self::TemplateName => Some(Parameter::String(template.name.0.clone())),
			
		}
	}
	
	pub fn from_json(value: &Value) -> PResult<Self> {
		if !value.is_array() {
			return Ok(Self::Constant(Parameter::guess_from_json(value).ok_or(perr!("invalid component parameter {:?}", value))?));
		}
		let paramvalue = value.get(1).ok_or(perr!("index 1 not in component parameter"))?;
		let typename = value.get(0).ok_or(perr!("index 0 not in component parameter"))?.as_str().ok_or(perr!("compparam type not a string"))?;
		if let Some(paramtype) = ParameterType::from_str(typename) {
			Ok(Self::Constant(Parameter::from_typed_json(paramtype, paramvalue).ok_or_else(||
				perr!("failed to parse parameter constant: {:?} {:?}", paramtype, paramvalue)
			)?))
		} else {
			match typename {
				"A" | "arg" => {
					let argname = paramvalue.as_str().ok_or(perr!("argument parameter not a string"))?.to_string();
					Ok(Self::Argument(argname))
				},
				"random" => {
					let optionvalues = paramvalue.as_array().ok_or(perr!("random argument not an array"))?;
					let mut options = Vec::new();
					for option in optionvalues {
						options.push(Self::from_json(option)?)
					}
					Ok(Self::Random(options))
				},
				"concat" => {
					let values = paramvalue.as_array().ok_or(perr!("concat argument not an array"))?;
					let mut options = Vec::new();
					for option in values {
						options.push(Self::from_json(option)?)
					}
					Ok(Self::Concat(options))
				},
				"if" => {
					Ok(Self::If(
						Box::new(Self::from_json(paramvalue.get(0).ok_or(perr!("if does not have condition"))?)?),
						Box::new(Self::from_json(paramvalue.get(1).ok_or(perr!("if does not have then value"))?)?),
						Box::new(Self::from_json(paramvalue.get(2).ok_or(perr!("if does not have else value"))?)?)
					))
				}
				"self" => Ok(Self::TemplateSelf),
				"name" => Ok(Self::TemplateName),
				_ => Err(perr!("unknown compparam type '{}'", typename))
			}
		}
	}
	
	pub fn get_type(&self, arguments: &[(String, ParameterType, Option<Parameter>)]) -> Result<ParameterType>{
		Ok(match self {
			Self::Constant(param) => param.paramtype(),
			Self::Argument(argname) => arguments.iter().find(|(n, _t, _d)| n == argname).ok_or(aerr!("unknown argument name {} in {:?}", argname, arguments))?.1,
			Self::Random(options) => {
				let typ: ParameterType = options.get(0).ok_or(aerr!("random has no options"))?.get_type(arguments)?;
				for param in options {
					if param.get_type(arguments)? != typ {
						return Err(aerr!("inconsistent parameter types in random"));
					}
				}
				typ
			},
			Self::If(condition, thenval, elseval) => {
				if condition.get_type(arguments)? != ParameterType::Bool {
					return Err(aerr!("if condition is not a bool"));
				}
				let typ: ParameterType = thenval.get_type(arguments)?;
				if elseval.get_type(arguments)? != typ {
					return Err(aerr!("inconsistent parameter types in if"));
				}
				typ
			},
			Self::Concat(_s) => ParameterType::String,
			Self::TemplateSelf => ParameterType::Template,
			Self::TemplateName => ParameterType::String
		})
	}
}
