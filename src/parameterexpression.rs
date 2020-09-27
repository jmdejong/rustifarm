
use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Deserializer};
use crate::{
	parameter::{Parameter, ParameterType},
	Template,
	template::{EntityType},
	Result as AnyResult,
	aerr,
};

const MAX_NESTING: usize = 5;


#[derive(Debug, PartialEq, Clone)]
pub enum ParameterExpression {
	Constant(Parameter),
	List(Vec<ParameterExpression>),
	#[allow(dead_code)] // rustc bug does not know that this variant is used: https://github.com/rust-lang/rust/issues/68408
	Template{name: EntityType, kwargs: HashMap<String, ParameterExpression>, save: Option<bool>, clan: Option<String>},
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
			Self::Template{name, kwargs, save, clan} => {
				Some(Parameter::Template(Template{
					name: name.clone(),
					save: *save,
					kwargs: kwargs
						.iter()
						.map(
							|(k, v)|
							Some((k.clone(), v.evaluate_(arguments, template, nesting+1)?)))
						.collect::<Option<HashMap<String, Parameter>>>()?,
					clan: clan.clone()
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
	
	#[allow(dead_code)]
	pub fn get_type(&self, arguments: &[(String, ParameterType, Option<Parameter>)]) -> AnyResult<ParameterType>{
		Ok(match self {
			Self::Constant(param) => param.paramtype(),
			Self::List(_) => ParameterType::List,
			Self::Template{name: _, kwargs: _, save: _, clan: _} => ParameterType::Template,
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


#[derive(Debug, PartialEq, Clone, Deserialize)]
enum DynamicParameterExpressionSave {
	#[serde(rename = "$arg")]
	Argument(String),
	#[serde(rename = "$random")]
	Random(Vec<ParameterExpression>),
	#[serde(rename = "$concat")]
	Concat(Vec<ParameterExpression>),
	#[serde(rename = "$if")]
	If(Box<ParameterExpression>, Box<ParameterExpression>, Box<ParameterExpression>),
	#[serde(rename = "$self")]
	TemplateSelf,
	#[serde(rename = "$name")]
	TemplateName
}
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
enum ParameterExpressionSave {
	List(Vec<ParameterExpression>),
	Template {
		#[serde(rename = "$template")]
		name: EntityType,
		#[serde(rename="__save__", default, skip_serializing_if = "Option::is_none")]
		save: Option<bool>,
		#[serde(rename="__clan__", default, skip_serializing_if = "Option::is_none")]
		clan: Option<String>,
		#[serde(flatten)]
		kwargs: HashMap<String, ParameterExpression>
	},
	Dynamic(DynamicParameterExpressionSave),
	Constant(Parameter),
}
use ParameterExpressionSave as PES;
use DynamicParameterExpressionSave as DPES;

impl<'de> Deserialize<'de> for ParameterExpression {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let save = ParameterExpressionSave::deserialize(deserializer)?;
		Ok(match save {
			PES::List(params) => Self::List(params),
			PES::Template{name, save, clan, kwargs} => Self::Template{name, save, kwargs, clan},
			PES::Dynamic(DPES::Argument(name)) => Self::Argument(name),
			PES::Dynamic(DPES::Random(items)) => Self::Random(items),
			PES::Dynamic(DPES::Concat(items)) => Self::Concat(items),
			PES::Dynamic(DPES::If(condition, ifpart, elsepart)) => Self::If(condition, ifpart, elsepart),
			PES::Dynamic(DPES::TemplateSelf) => Self::TemplateSelf,
			PES::Dynamic(DPES::TemplateName) => Self::TemplateName,
			PES::Constant(param) => Self::Constant(param)
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::ParameterExpression as PE;
	use crate::hashmap;
	use serde_json::json;
	
	#[test]
	fn test_desrialize(){
		assert_eq!(
			PE::deserialize(json!("hello")).unwrap(),
			PE::Constant(Parameter::String("hello".to_string()))
		);
		assert_eq!(
			PE::deserialize(json!({"$arg": "hello"})).unwrap(),
			PE::Argument("hello".to_string())
		);
		assert_eq!(
			PE::deserialize(json!(["hello", 3])).unwrap(),
			PE::List(vec![
				PE::Constant(Parameter::String("hello".to_string())),
				PE::Constant(Parameter::Int(3))
			])
		);
	}
	#[test]
	fn test_templates(){
		assert_eq!(
			PE::deserialize(json!({"$template": "radish", "health": 10})).unwrap(),
			PE::Template{
				name: EntityType("radish".to_string()),
				kwargs: hashmap!{"health".to_string() => PE::Constant(Parameter::Int(10))},
				save: None,
				clan: None
			}
		);
		assert_eq!(
			PE::deserialize(json!({":template": "radish", "health": 10})).unwrap(),
			PE::Constant(Parameter::Template(Template{
				name: EntityType("radish".to_string()),
				kwargs: hashmap!{"health".to_string() => Parameter::Int(10)},
				save: None,
				clan: None
			}))
		);
		assert_eq!(
			PE::deserialize(json!({"$template": "radish", "health": {"$if": [{"$arg": "is_eldritch"}, 20, 3]}})).unwrap(),
			PE::Template{
				name: EntityType("radish".to_string()),
				kwargs: hashmap!{"health".to_string() => PE::If(
					Box::new(PE::Argument("is_eldritch".to_string())),
					Box::new(PE::Constant(Parameter::Int(20))),
					Box::new(PE::Constant(Parameter::Int(3)))
				)},
				save: None,
				clan: None
			}
		);
	}
}

