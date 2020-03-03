
use std::collections::HashMap;
use serde_json::{Value, json};
use crate::{
	componentparameter::ComponentParameter,
	parameter::{Parameter, ParameterType},
	componentwrapper::{ComponentWrapper, ComponentType},
	components::Serialise,
	hashmap,
	Template
};

type ArgumentDef = (String, ParameterType, Option<Parameter>);

#[derive(Debug, PartialEq, Clone)]
pub struct Assemblage {
	pub arguments: Vec<ArgumentDef>,
	pub components: Vec<(ComponentType, HashMap<String, ComponentParameter>)>,
	pub save: bool
}

impl Assemblage {


	fn parse_definition_arguments(args: &Value) -> Result<Vec<ArgumentDef>, &'static str> {
		let mut arguments: Vec<ArgumentDef> = Vec::new();
		for arg in args.as_array().ok_or("arguments is not an array")? {
			let tup = arg.as_array().ok_or("argument is not an array")?;
			let key = tup.get(0).ok_or("argument has no name")?.as_str().ok_or("argument name is not a string")?.to_string();
			let typ = ParameterType::from_str(tup.get(1).ok_or("argument has no type")?.as_str().ok_or("argument type not a string")?).ok_or("failed to parse argument type")?;
			let def = tup.get(2).ok_or("argument has no default")?;
			if def.is_null() {
				arguments.push((key.clone(), typ, None));
			} else {
				arguments.push((key.clone(), typ, Some(Parameter::from_typed_json(typ, def).ok_or("invalid argument default")?)));
			}
		}
		Ok(arguments)
	}
	
	fn parse_definition_components(comps: &Value) -> Result<Vec<(ComponentType, HashMap<String, ComponentParameter>)>, &'static str> {
		let mut components = Vec::new();
		for tup in comps.as_array().ok_or("components is not a json array")? {
			if let Some(name) = tup.as_str() {
				components.push((ComponentType::from_str(name).ok_or("not a valid componenttype")?, HashMap::new()));
			} else {
				let comptype = ComponentType::from_str(tup
					.get(0).ok_or("index 0 not in component")?
					.as_str().ok_or("component name not a string")?
				).ok_or("not a valid componenttype")?;
				let mut parameters: HashMap<String, ComponentParameter> = HashMap::new();
				for (key, value) in tup.get(1).ok_or("index 1 not in component")?.as_object().ok_or("component parameters not a json object")? {
					let param = ComponentParameter::from_json(value)?;
					parameters.insert(key.clone(), param);
				}
				components.push((comptype, parameters));
			}
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
	
	pub fn from_json(val: &Value) -> Result<Self, &'static str>{
		let mut assemblage = Self {
			arguments: Self::parse_definition_arguments(val.get("arguments").unwrap_or(&json!([])))?,
			components: Self::parse_definition_components(val.get("components").ok_or("property 'components' not found")?)?,
			save: val.get("save").unwrap_or(&json!(true)).as_bool().ok_or("assemblage save not a bool")?
		};
		// visible component is so common that shortcuts are very helpful
		if let Some(spritename) = val.get("sprite") {
			let height = val.get("height").ok_or("defining a sprite requires also defining a height")?;
			let name = val.get("name").unwrap_or(spritename);
			assemblage.components.push((
				ComponentType::Visible,
				hashmap!(
					"sprite".to_string() => ComponentParameter::Constant(
						Parameter::String(spritename.as_str().ok_or("sprite not a string")?.to_string())
					),
					"height".to_string() => ComponentParameter::Constant(
						Parameter::Float(height.as_f64().ok_or("height not a float")?)
					),
					"name".to_string() => ComponentParameter::Constant(
						Parameter::String(name.as_str().ok_or("name not a string")?.to_string())
					)
				)
			));
		}
		assemblage.validate()?;
		Ok(assemblage)
	}
	
	fn prepare_arguments(&self, args: &[Parameter], kwargs: &HashMap<String, Parameter>) -> Result<HashMap<&str, Parameter>, &'static str> {
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

	pub fn instantiate(&self, template: &Template) -> Result<Vec<ComponentWrapper>, &'static str>{
		let args = &template.args;
		let kwargs = &template.kwargs;
		let mut components: Vec<ComponentWrapper> = Vec::new();
		let arguments = self.prepare_arguments(args, kwargs)?;
		for (comptype, compparams) in &self.components {
			let mut compargs: HashMap<&str, Parameter> = HashMap::new();
			for (name, param) in compparams {
				compargs.insert(name.as_str(), param.evaluate(&arguments).ok_or("argument not found")?);
			}
			components.push(ComponentWrapper::load_component(*comptype, compargs).ok_or("failed to load component")?);
		}
		if template.save && self.save {
			components.push(ComponentWrapper::Serialise(Serialise{template: template.clone()}));
		}
		Ok(components)
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use serde_json::json;
	
	
	#[test]
	fn empty_assemblage_from_json() {
		assert_eq!(
				Assemblage::from_json(&json!({
				"arguments": [],
				"components": []
			})).unwrap(),
			Assemblage{
				arguments: vec![],
				components: vec![],
				save: true
			}
		);
	}
	
	#[test]
	fn grass_from_json(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string", "grass1"]
				],
				"components": [
					["Visible", {
						"sprite": ["arg", "sprite"],
						"height": ["float", 0.1],
						"name": ["string", "grass"]
					}]
				]
			})).unwrap();
		let constructed = Assemblage{
				arguments: vec![("sprite".to_string(), ParameterType::String, Some(Parameter::String("grass1".to_string())))],
				components: vec![
					(ComponentType::Visible, hashmap!(
						"sprite".to_string() => ComponentParameter::Argument("sprite".to_string()),
						"height".to_string() => ComponentParameter::Constant(Parameter::Float(0.1)),
						"name".to_string() => ComponentParameter::Constant(Parameter::String("grass".to_string()))
					))
				],
				save: true
			};
		assert_eq!(result, constructed);
	}
	
	#[test]
	fn invalid_component_name(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string", null]
				],
				"components": [
					["visible", { // no capital so invalid
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1],
						"name": ["string", "grass"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "not a valid componenttype");
	}
	
	
	
	#[test]
	fn invalid_parameter_type(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string", "grass1"]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["string", "0.1"],
						"name": ["string", "grass"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "parameter type incorrect");
	}
	
	#[test]
	fn unknown_argument_name(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string", "grass1"]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprits"],
						"height": ["float", 0.1],
						"name": ["string", "grass"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "unknown argument name");
	}
	
	#[test]
	fn wrong_argument_type(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "int", 1]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1],
						"name": ["string", "grass"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "parameter type incorrect");
	}
	
	
	
	#[test]
	fn wrong_argument_default(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string", 1]
				],
				"components": [
					["Visible", {
						"sprite": ["A", "sprite"],
						"height": ["float", 0.1],
						"name": ["string", "grass"]
					}]
				]
			})).unwrap_err();
		assert_eq!(result, "invalid argument default");
	}
	
	
	#[test]
	fn null_argument(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string", null]
				],
				"components": [
					["Visible", {
						"sprite": ["arg", "sprite"],
						"height": ["float", 0.1],
						"name": ["arg", "sprite"]
					}]
				]
			})).unwrap();
		let constructed = Assemblage{
				arguments: vec![("sprite".to_string(), ParameterType::String, None)],
				components: vec![
					(ComponentType::Visible, hashmap!(
						"sprite".to_string() => ComponentParameter::Argument("sprite".to_string()),
						"height".to_string() => ComponentParameter::Constant(Parameter::Float(0.1)),
						"name".to_string() => ComponentParameter::Argument("sprite".to_string())
					))
				],
				save: true
			};
		assert_eq!(result, constructed);
	}
}
