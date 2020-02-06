
use std::collections::HashMap;
use specs::{Builder, EntityBuilder};
use serde_json::Value;

use crate::components::{Visible, Blocking, Played};
use crate::pos::Pos;
use crate::hashmap;


#[derive(Clone)]
pub enum CompWrapper{
	Visible(Visible),
	Blocking(Blocking),
	Player(Played)
}

impl CompWrapper {

	pub fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
		match self.clone() {
			Self::Visible(c) => builder.with(c),
			Self::Blocking(c) => builder.with(c),
			Self::Player(c) => builder.with(c)
		}
	}

	pub fn load_component(comptype: ComponentType, mut parameters: HashMap<&str, Parameter>) -> Option<CompWrapper> {
		match comptype {
			ComponentType::Visible => Some(CompWrapper::Visible(Visible{
				sprite: parameters.remove("sprite")?.as_str()?.to_string(),
				height: parameters.remove("height")?.as_f64()?
			})),
			ComponentType::Blocking => Some(CompWrapper::Blocking(Blocking)),
			ComponentType::Player => Some(CompWrapper::Player(Played::new(
				parameters.remove("name")?.as_str()?.to_string()
			)))
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ComponentType {
	Visible,
	Blocking,
	Player
}

impl ComponentType {
	
	pub fn from_str(typename: &str) -> Option<ComponentType>{
		match typename {
			"Visible" => Some(ComponentType::Visible),
			"Blocking" => Some(ComponentType::Blocking),
			"Player" => Some(ComponentType::Player),
			_ => None
		}
	}
	
	pub fn parameters(&self) -> HashMap<&str, ParamType> {
		match self {
			ComponentType::Visible => hashmap!("sprite" => ParamType::String, "height" => ParamType::Float),
			ComponentType::Blocking => HashMap::new(),
			ComponentType::Player => hashmap!("name" => ParamType::String)
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Parameter {
	String(String),
	Int(i64),
// 	Pos(Pos),
	Float(f64)
}

impl Parameter {

	pub fn from_typed_json(typ: ParamType, val: &Value) -> Option<Parameter>{
		match typ {
			ParamType::String => Some(Parameter::String(val.as_str()?.to_string())),
			ParamType::Int => Some(Parameter::Int(val.as_i64()?)),
			ParamType::Float => Some(Parameter::Float(val.as_f64()?))
		}
	}
	
	pub fn paramtype(&self) -> ParamType {
		match self {
			Parameter::String(_) => ParamType::String,
			Parameter::Int(_) => ParamType::Int,
			Parameter::Float(_) => ParamType::Float
		}
	}
	
	pub fn from_json(val: &Value) -> Option<Parameter> {
		Parameter::from_typed_json(ParamType::from_str(val.get(0)?.as_str()?)?, val.get(1)?)
	}
	
	pub fn as_str(&self) -> Option<&str> {
		if let Parameter::String(str) = self {
			Some(str)
		} else {
			None
		}
	}
	
	pub fn as_string(&self) -> Option<String> {
		Some(self.as_str()?.to_string())
	}
	
	pub fn as_i64(&self) -> Option<i64> {
		if let Parameter::Int(num) = self {
			Some(*num)
		} else {
			None
		}
	}
	
	pub fn as_f64(&self) -> Option<f64> {
		if let Parameter::Float(num) = self {
			Some(*num)
		} else {
			None
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
	String,
// 	Pos,
	Int,
	Float
}

impl ParamType {
	
	pub fn from_str(typename: &str) -> Option<ParamType>{
		match typename {
			"string" => Some(ParamType::String),
			"int" => Some(ParamType::Int),
			"float" => Some(ParamType::Float),
			_ => None
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Template {
	pub arguments: Vec<(String, ParamType, Option<Parameter>)>,
	pub components: Vec<(ComponentType, HashMap<String, CompParam>)>
}

impl Template {


	fn parse_definition_arguments(args: &Value) -> Result<Vec<(String, ParamType, Option<Parameter>)>, &'static str> {
		let mut arguments: Vec<(String, ParamType, Option<Parameter>)> = Vec::new();
		for arg in args.as_array().ok_or("arguments is not an array")? {
			let tup = arg.as_array().ok_or("argument is not an array")?;
			let key = tup.get(0).ok_or("argument has no name")?.as_str().ok_or("argument name is not a string")?.to_string();
			let typ = ParamType::from_str(tup.get(1).ok_or("argument has no type")?.as_str().ok_or("argument type not a string")?).ok_or("failed to parse argument type")?;
			let def = tup.get(2).ok_or("argument has no default")?;
			if def.is_null() {
				arguments.push((key.clone(), typ, None));
			} else {
				arguments.push((key.clone(), typ, Some(Parameter::from_typed_json(typ, def).ok_or("invalid argument default")?)));
			}
		}
		Ok(arguments)
	}
	
	fn parse_definition_components(comps: &Value) -> Result<Vec<(ComponentType, HashMap<String, CompParam>)>, &'static str> {
		let mut components = Vec::new();
		for tup in comps.as_array().ok_or("components is not a json array")? {
			let comptype = ComponentType::from_str(tup
				.get(0).ok_or("index 0 not in component")?
				.as_str().ok_or("component name not a string")?
			).ok_or("not a valid componenttype")?;
			let mut parameters: HashMap<String, CompParam> = HashMap::new();
			for (key, value) in tup.get(1).ok_or("index 1 not in component")?.as_object().ok_or("component parameters not a json object")? {
				let param = CompParam::from_json(value)?;
				parameters.insert(key.clone(), param);
			}
			components.push((comptype, parameters));
		}
		Ok(components)
	}
	
	fn validate(&self) -> Result<(), &'static str> {
		for (comptype, parameters) in &self.components {
			for (paramname, paramtype) in comptype.parameters() {
				let param = parameters.get(paramname).ok_or("missing parameter")?;
				let actualtype = param.get_type(&self.arguments)?;
				if actualtype != paramtype {
					return Err("parameter type incorrect");
				}
			}
		}
		Ok(())
	}
	
	pub fn from_json(val: &Value) -> Result<Template, &'static str>{
		let template = Template {
			arguments: Self::parse_definition_arguments(val.get("arguments").ok_or("property 'arguments' not found")?)?,
			components: Self::parse_definition_components(val.get("components").ok_or("property 'components' not found")?)?
		};
		template.validate()?;
		Ok(template)
	}
	
	fn prepare_arguments(&self, args: Vec<Parameter>, kwargs: HashMap<String, Parameter>) -> Result<HashMap<&str, Parameter>, &str> {
		let mut arguments: HashMap<&str, Parameter> = HashMap::new();
		for (idx, (name, typ, def)) in self.arguments.iter().enumerate() {
			let value: Option<Parameter> = {
				if let Some(val) = kwargs.get(name) {
					Some(val.clone())
				} else if let Some(val) = args.get(idx) {
					Some(val.clone())
				} else if let Some(val) = def {
					Some(val.clone())
				} else {
					None
				}
			};
			let param = value.ok_or("argument has no value")?;
			if param.paramtype() != *typ {
				return Err("argument has incorrect type");
			}
			arguments.insert(name, param);
		}
		Ok(arguments)
	}

	pub fn instantiate(&self, args: Vec<Parameter>, kwargs: HashMap<String, Parameter>) -> Result<Vec<CompWrapper>, &str>{
		let mut components: Vec<CompWrapper> = Vec::new();
		let arguments = self.prepare_arguments(args, kwargs)?;
		for (comptype, compparams) in &self.components {
			let mut compargs: HashMap<&str, Parameter> = HashMap::new();
			for (name, param) in compparams {
				compargs.insert(name.as_str(), param.evaluate(&arguments).ok_or("argument not found")?);
			}
			components.push(CompWrapper::load_component(*comptype, compargs).ok_or("failed to load component")?);
		}
		Ok(components)
	}
}


#[derive(Debug, PartialEq)]
pub enum CompParam {
	Constant(Parameter),
	Argument(String)
}

impl CompParam {
	pub fn evaluate(&self, arguments: &HashMap<&str, Parameter>) -> Option<Parameter> {
		match self {
			CompParam::Constant(val) => {
				Some(val.clone())
			},
			CompParam::Argument(argname) => {
				Some(arguments.get(argname.as_str())?.clone())
			}
		}
	}
	
	pub fn from_json(value: &Value) -> Result<Self, &'static str> {
		let paramvalue = value.get(1).ok_or("index 0 not in component parameter")?;
		let typename = value.get(0).ok_or("index 0 not in component parameter")?.as_str().ok_or("compparam type not a string")?;
		if let Some(paramtype) = ParamType::from_str(typename) {
			Ok(CompParam::Constant(Parameter::from_typed_json(paramtype, paramvalue).ok_or("failed to parse parameter constant")?))
		} else {
			match typename {
				"A" | "arg" => {
					let argname = paramvalue.as_str().ok_or("argument parameter not a string")?.to_string();
					Ok(CompParam::Argument(argname))
				},
				_ => Err("unknown compparam type")
			}
		}
	}
	
	pub fn get_type(&self, arguments: &Vec<(String, ParamType, Option<Parameter>)>) -> Result<ParamType, &'static str>{
		Ok(match self {
			Self::Constant(param) => param.paramtype(),
			Self::Argument(argname) => arguments.iter().find(|(n, _t, _d)| n == argname).ok_or("unknown argument name")?.1
		})
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	
	#[test]
	fn empty_template_from_json() {
		assert_eq!(
			Template::from_json(&json!({
				"arguments": [],
				"components": []
			})).unwrap(),
			Template{
				arguments: vec![],
				components: vec![]
			}
		);
	}
	
	#[test]
	fn grass_from_json(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "string", "grass1"]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1]
					}]
				]
			})).unwrap();
		let constructed = Template{
				arguments: vec![("sprite".to_string(), ParamType::String, Some(Parameter::String("grass1".to_string())))],
				components: vec![
					(ComponentType::Visible, hashmap!(
						"sprite".to_string() => CompParam::Argument("sprite".to_string()),
						"height".to_string() => CompParam::Constant(Parameter::Float(0.1))
					))
				]
			};
		assert_eq!(result, constructed);
	}
	
	#[test]
	fn invalid_component_name(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "string", null]
				],
				"components": [
					["visible", { // no capital so invalid
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "not a valid componenttype");
	}
	
	
	
	#[test]
	fn invalid_parameter_type(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "string", "grass1"]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["string", "0.1"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "parameter type incorrect");
	}
	
	#[test]
	fn unknown_argument_name(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "string", "grass1"]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprits"],
						"height": ["float", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "unknown argument name");
	}
	
	#[test]
	fn wrong_argument_type(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "int", 1]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "parameter type incorrect");
	}
	
	
	
	#[test]
	fn wrong_argument_default(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "string", 1]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprits"],
						"height": ["float", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "invalid argument default");
	}
	
	
	#[test]
	fn null_argument(){
		let result = Template::from_json(&json!({
				"arguments": [
					["sprite", "string", null]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1]
					}]
				]
			})).unwrap();
		let constructed = Template{
				arguments: vec![("sprite".to_string(), ParamType::String, None)],
				components: vec![
					(ComponentType::Visible, hashmap!(
						"sprite".to_string() => CompParam::Argument("sprite".to_string()),
						"height".to_string() => CompParam::Constant(Parameter::Float(0.1))
					))
				]
			};
		assert_eq!(result, constructed);
	}
}



