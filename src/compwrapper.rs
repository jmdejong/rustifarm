
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

	pub fn build<'a>(&self, builder: specs::EntityBuilder<'a>) -> specs::EntityBuilder<'a> {
		match self.clone() {
			Self::Visible(c) => builder.with(c),
			Self::Blocking(c) => builder.with(c),
			Self::Player(c) => builder.with(c)
		}
	}
	
// 	pub fn parse_component(data: Value) -> Option<CompWrapper> {
// 		let a = data.as_array()?;
// 		if a.len() != 2 {
// 			return None
// 		}
// 		let typename = a[0].as_str()?;
// 		let params: HashMap<&str, &Parameter> = a[1].as_object()?.into_iter().map(|(key, val)| (key.as_str(), val)).collect();
// 		Self::load_component(typename, params)
// 	}

	pub fn load_component(comptype: ComponentType, mut parameters: HashMap<&str, &Parameter>) -> Option<CompWrapper> {
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

#[derive(Debug, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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
	pub arguments: Vec<(String, Option<Parameter>)>,
	pub components: Vec<(ComponentType, HashMap<String, CompParam>)>
}

impl Template {

	pub fn from_json(val: Value) -> Result<Template, &'static str>{
		let mut arguments: Vec<(String, Option<Parameter>)> = Vec::new();
		for arg in val.get("arguments").ok_or("property 'arguments' not found")?.as_array().ok_or("arguments is not a json object")? {
			let tup = arg.as_array().ok_or("argument is not an array")?;
			let key = tup.get(0).ok_or("argument has no index name")?.as_str().ok_or("argument name is not a string")?.to_string();
			let value = tup.get(1).ok_or("argument has only name")?;
			if value.is_null() {
				arguments.push((key.clone(), None));
			} else {
				arguments.push((key.clone(), Some(Parameter::from_json(value).ok_or("invalid argument default")?)));
			}
		}
		let mut components = Vec::new();
		for tup in val.get("components").ok_or("property 'arguments' not found")?.as_array().ok_or("arguments is not a json object")? {
			let comptype = ComponentType::from_str(tup
				.get(0).ok_or("index 0 not in component")?
				.as_str().ok_or("component name not a string")?
			).ok_or("not a valid componenttype")?;
			let mut parameters: HashMap<String, CompParam> = HashMap::new();
			for (key, value) in tup.get(1).ok_or("index 1 not in component")?.as_object().ok_or("component parameters not a json object")? {
				let paramtype: ParamType = comptype.parameters().remove(key.as_str()).ok_or("unknown parameter name")?;
				let paramvalue = value.get(1).ok_or("index 0 not in component parameter")?;
				let param = match value.get(0).ok_or("index 0 not in component parameter")?.as_str().ok_or("compparam type not a string")? {
					"C" => Ok(CompParam::Constant(
						Parameter::from_typed_json(paramtype, paramvalue).ok_or("failed to parse parameter constant")?
					)),
					"A" => {
						let argname = paramvalue.as_str().ok_or("argument parameter not a string")?.to_string();
						let arg = arguments.iter().find(|(a, d)| a == &argname).ok_or("unknown argument name")?;
						match &arg.1 {
							Some(param) => {
								if param.paramtype() == paramtype {
									Ok(CompParam::Argument(argname))
								} else {
									Err("wrong argument type")
								}
							},
							None => Ok(CompParam::Argument(argname))
						}
					},
					_ => Err("unknown compparam type")
				};
				parameters.insert(key.clone(), param?);
			}
			components.push((comptype, parameters));
		}
		Ok(Template {
			arguments,
			components
		})
	}

	pub fn instantiate(&self, args: Vec<Parameter>, kwargs: HashMap<String, Parameter>) -> Option<Vec<CompWrapper>>{
		let mut components: Vec<CompWrapper> = Vec::new();
		for (comptype, compparams) in &self.components {
			let mut compargs: HashMap<&str, &Parameter> = HashMap::new();
			for (name, param) in compparams {
				match param {
					CompParam::Constant(val) => {compargs.insert(name.as_str(), &val); Some(())},
					CompParam::Argument(argname) => {
						if let Some(argval) = kwargs.get(argname.as_str()) {
							compargs.insert(name.as_str(), argval);
							Some(())
						} else if let Some(idx) = self.arguments.iter().position(|(x, d)| x == name){
							if idx < args.len() {
								compargs.insert(name.as_str(), &args[idx]);
								Some(())
							} else {
								println!("positional argument out of range");
								None
							}
						} else {
							println!("can't find parameter value, comptype: {:?}, name: {}, argname: {}", comptype, name, argname);
							None
						}
					}
				}?;
			}
			components.push(CompWrapper::load_component(*comptype, compargs)?);
		}
		Some(components)
	}
}


#[derive(Debug, PartialEq)]
pub enum CompParam {
	Constant(Parameter),
	Argument(String)
}


#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	use std::collections::HashMap;
	
	
	#[test]
	fn empty_template_from_json() {
		assert_eq!(
			Template::from_json(json!({
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
		let result = Template::from_json(json!({
				"arguments": [
					["sprite", ["string", "grass1"]]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["C", 0.1]
					}]
				]
			})).unwrap();
		let constructed = Template{
				arguments: vec![("sprite".to_string(), Some(Parameter::String("grass1".to_string())))],
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
		let result = Template::from_json(json!({
				"arguments": [
					["sprite", ["string", "grass1"]]
				],
				"components": [
					["visible", { // no capital so invalid
						"sprite": ["A", "sprite"],
						"height": ["C", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "not a valid componenttype");
	}
	
	
	
	#[test]
	fn invalid_parameter_type(){
		let result = Template::from_json(json!({
				"arguments": [
					["sprite", ["string", "grass1"]]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["C", "0.1"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "failed to parse parameter constant");
	}
	
	#[test]
	fn unknown_argument_name(){
		let result = Template::from_json(json!({
				"arguments": [
					["sprite", ["string", "grass1"]]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprits"],
						"height": ["C", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "unknown argument name");
	}
	
	#[test]
	fn wrong_argument_type(){
		let result = Template::from_json(json!({
				"arguments": [
					["sprite", ["int", 1]]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["C", 0.1]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "wrong argument type");
	}
}



