
use std::collections::HashMap;
use rand::Rng;
use serde_json::{Value, json};
use serde::{Deserialize, Deserializer, de, Serialize};
use crate::{
	parameter::{Parameter, ParameterType},
	Template,
	template::{EntityType},
	Result as AnyResult,
	aerr,
	PResult,
	perr
};

const MAX_NESTING: usize = 5;


#[derive(Debug, PartialEq, Clone)]
pub enum ParameterExpression {
	Constant(Parameter),
	List(Vec<ParameterExpression>),
	#[allow(dead_code)] // rustc bug does not know that this variant is used: https://github.com/rust-lang/rust/issues/68408
	Template{name: EntityType, kwargs: HashMap<String, ParameterExpression>, save: Option<bool>},
	Argument(String),
	Random(Vec<ParameterExpression>),
	Concat(Vec<ParameterExpression>),
	If(Box<ParameterExpression>, Box<ParameterExpression>, Box<ParameterExpression>),
	TemplateSelf,
	TemplateName
}

impl ParameterExpression {

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
			Self::List(values) => {
				Some(Parameter::List(values.iter().map(|v| v.evaluate_(arguments, template, nesting+1)).collect::<Option<Vec<Parameter>>>()?))
			}
			Self::Template{name, kwargs, save} => {
				Some(Parameter::Template(Template{
					name: name.clone(),
					args: Vec::new(),
					save: *save,
					kwargs: kwargs
						.iter()
						.map(
							|(k, v)|
							Some((k.clone(), v.evaluate_(arguments, template, nesting+1)?)))
						.collect::<Option<HashMap<String, Parameter>>>()?
				}))
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
			return Ok(Self::Constant(Parameter::guess_from_json(value)?));
		}
		let paramvalue = value.get(1).ok_or(perr!("index 1 not in component parameter"))?;
		let typename = value.get(0).ok_or(perr!("index 0 not in component parameter"))?.as_str().ok_or(perr!("compparam type not a string"))?;
		match typename {
			"string" | "int" | "float" | "bool" | "pos" => {
				let paramtype = ParameterType::from_str(typename).expect(&format!("unknown parameter type {:?}", typename));
				Ok(Self::Constant(Parameter::from_typed_json(paramtype, paramvalue)?))
			}
			"list" => {
				let values = paramvalue.as_array().ok_or(perr!("random argument not an array"))?;
				let mut entries = Vec::new();
				for entry in values {
					entries.push(Self::from_json(entry)?)
				}
				Ok(Self::List(entries))
			}
			"template" => {
				match paramvalue {
					Value::String(s) => Ok(Self::Template{
						name: EntityType(s.clone()),
						kwargs: HashMap::new(),
						save: None
					}),
					Value::Object(o) => {
						let name = EntityType(o.get("type").ok_or(perr!("template doesn't have 'type'"))?.as_str().ok_or(perr!("template type not a string"))?.to_string());
						let mut kwargs = HashMap::new();
						for (key, arg) in o.get("kwargs").unwrap_or(&json!({})).as_object().ok_or(perr!("template kwargs not a json object"))? {
							kwargs.insert(key.to_string(), Self::from_json(arg)?);
						}
						let save = match o.get("save") {
							Some(Value::Bool(b)) if *b => Some(true),
							Some(Value::Bool(_b)) => Some(false),
							None => None,
							_ => {return Err(perr!("save not a bool"))}
						};
						Ok(Self::Template{name, kwargs, save})
					}
					_ => return Err(perr!("invalid template {:?}", paramvalue))
				}
			}
			"A" | "arg" => {
				let argname = paramvalue.as_str().ok_or(perr!("argument parameter not a string"))?.to_string();
				Ok(Self::Argument(argname))
			}
			"random" => {
				let optionvalues = paramvalue.as_array().ok_or(perr!("random argument not an array"))?;
				let mut options = Vec::new();
				for option in optionvalues {
					options.push(Self::from_json(option)?)
				}
				Ok(Self::Random(options))
			}
			"concat" => {
				let values = paramvalue.as_array().ok_or(perr!("concat argument not an array"))?;
				let mut options = Vec::new();
				for option in values {
					options.push(Self::from_json(option)?)
				}
				Ok(Self::Concat(options))
			}
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
	
	#[allow(dead_code)]
	pub fn get_type(&self, arguments: &[(String, ParameterType, Option<Parameter>)]) -> AnyResult<ParameterType>{
		Ok(match self {
			Self::Constant(param) => param.paramtype(),
			Self::List(_) => ParameterType::List,
			Self::Template{name: _, kwargs: _, save: _} => ParameterType::Template,
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

// impl Serialize for ParameterExpression {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where S: Serializer {
// 		self.to_json().serialize(serializer)
// 	}
// }
impl<'de> Deserialize<'de> for ParameterExpression {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		Self::from_json(&Value::deserialize(deserializer)?).map_err(|e| de::Error::custom(e.text))
	}
}

