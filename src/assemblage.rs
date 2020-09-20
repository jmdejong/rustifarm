
use std::collections::HashMap;
use serde_json::{Value, json, value};
use crate::{
	parameterexpression::ParameterExpression,
	parameter::{Parameter, ParameterType},
	componentwrapper::{ComponentWrapper, ComponentType},
	components::Serialise,
	Template,
	Result,
	aerr,
	PResult,
	perr
};

type ArgumentDef = (String, ParameterType, Option<Parameter>);

#[derive(Debug, PartialEq, Clone)]
pub struct Assemblage {
	pub arguments: Vec<ArgumentDef>,
	pub components: Vec<(ComponentType, HashMap<String, ParameterExpression>)>,
	pub save: bool,
	pub extract: Vec<(String, ComponentType, String)>
}

impl Assemblage {


	fn parse_definition_arguments(args: &Value) -> PResult<Vec<ArgumentDef>> {
		let mut arguments: Vec<ArgumentDef> = Vec::new();
		for arg in args.as_array().ok_or(perr!("arguments is not an array"))? {
			let tup = arg.as_array().ok_or(perr!("argument is not an array"))?;
			let key = tup.get(0).ok_or(perr!("argument has no name"))?.as_str().ok_or(perr!("argument name is not a string"))?.to_string();
			let typ = ParameterType::from_str(tup.get(1).ok_or(perr!("argument has no type"))?.as_str().ok_or(perr!("argument type not a string"))?).ok_or(perr!("failed to parse argument type"))?;
			if let Some(def) = tup.get(2){
				arguments.push(
					(
						key.clone(),
						typ,
						Some(Parameter::from_typed_json(typ, def)?)
					)
				);
			} else  {
				arguments.push((key.clone(), typ, None));
			}
		}
		Ok(arguments)
	}
	
	fn parse_definition_components(comps: &[Value]) -> PResult<Vec<(ComponentType, HashMap<String, ParameterExpression>)>> {
		let mut components = Vec::new();
		for tup in comps {
			if let Some(name) = tup.as_str() {
				components.push((ComponentType::from_str(name).ok_or(perr!("{} not a valid componenttype", name))?, HashMap::new()));
			} else {
				let (name, params) = value::from_value::<(String, HashMap<String, Value>)>(tup.clone()).map_err(|e| perr!("invalid component definition: {:?}", e))?;
				let comptype = ComponentType::from_str(&name).ok_or(perr!("{} not a valid componenttype", name))?;
				let mut parameters: HashMap<String, ParameterExpression> = HashMap::new();
				for (key, value) in params.into_iter() {
					let param = ParameterExpression::from_json(&value)?;
					parameters.insert(key, param);
				}
				components.push((comptype, parameters));
			}
		}
		Ok(components)
	}
	
	
	fn preprocess(val: &Value) -> PResult<Vec<Value>> {
		let mut components = Vec::new();
		let name = if let Some(nameval) = val.get("name") {
				Some(nameval.as_str().ok_or(perr!("name not a string"))?.to_string())
			} else {None};
		
		// visible component is so common that shortcuts are very helpful
		if let Some(spritename) = val.get("sprite") {
			let sprite = spritename.as_str().ok_or(perr!("sprite not a string"))?.to_string();
			let height = val
				.get("height").ok_or(perr!("defining a sprite requires also defining a height"))?
				.as_f64().ok_or(perr!("height not a float"))?;
			components.push(json!(["Visible", {
				"name": ["string", name.clone().unwrap_or(sprite.clone())],
				"sprite": ["string", sprite],
				"height": ["float", height]
			}]));
		}
		// item component is common too
		if let Some(item) = val.get("item") {
			components.push(json!(["Item", {
				"item": ["string", item]
			}]));
		}
		// and so is flags
		if let Some(flags) = val.get("flags") {
			components.push(json!(["Flags", {
				"flags": ["list", flags]
			}]));
		}
		
		if let Some(substitute) = val.get("substitute") {
			components.push(json!(["Substitute", {"into": ["template", substitute]}]));
		}
		Ok(components)
	}
	
	pub fn from_json(val: &Value) -> PResult<Self>{
		let mut json_components: Vec<Value> = val
			.get("components")
			.unwrap_or(&json!([]))
			.as_array()
			.ok_or(perr!("components is not a json array"))?
			.to_vec();
		json_components.append(&mut Self::preprocess(val)?);
		let assemblage = Self {
			arguments: Self::parse_definition_arguments(val.get("arguments").unwrap_or(&json!([])))?,
			components: Self::parse_definition_components(&json_components)?,
			save: val.get("save").unwrap_or(&json!(true)).as_bool().ok_or(perr!("assemblage save not a bool"))?,
			extract: value::from_value::<HashMap<String, (ComponentType, String)>>(
					val.get("extract").unwrap_or(&json!({})).clone()
				).map_err(|e| perr!("invalid assemblage extract: {:?}", e))?
				.into_iter()
				.map(|(name, (comp, field))| (name, comp, field))
				.collect()
		};
		Ok(assemblage)
	}
	
	
	pub fn validate(&self) -> Result<()> {
		for (comptype, parameters) in &self.components {
			for paramname in comptype.parameters() {
				let _param = parameters.get(paramname).ok_or(aerr!("missing parameter {} for component {:?}", paramname, comptype))?;
				// todo: validate parameter types
			}
		}
		Ok(())
	}
	
	fn prepare_arguments(&self, args: &[Parameter], kwargs: &HashMap<String, Parameter>) -> Result<HashMap<&str, Parameter>> {
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
			let param = value.ok_or(aerr!("argument <{:?}> has no value", (idx, (name, typ, def))))?;
			if param.paramtype() != *typ {
				return Err(aerr!(
					"argument has incorrect type: {:?}, {:?}, {:?}",
					(idx, (name, typ, def)),
					param.paramtype(),
					param
				));
			}
			arguments.insert(name, param);
		}
		Ok(arguments)
	}

	pub fn instantiate(&self, template: &Template) -> Result<Vec<ComponentWrapper>>{
		let args = &template.args;
		let kwargs = &template.kwargs;
		let mut components: Vec<ComponentWrapper> = Vec::new();
		let arguments = self.prepare_arguments(args, kwargs)?;
		for (comptype, compparams) in &self.components {
			let mut compargs: HashMap<&str, Parameter> = HashMap::new();
			for (name, param) in compparams {
				compargs.insert(name.as_str(), param.evaluate(&arguments, template).ok_or(aerr!("argument not found"))?);
			}
			components.push(ComponentWrapper::load_component(*comptype, compargs)?);
		}
		if template.should_save() && self.save {
			components.push(ComponentWrapper::Serialise(Serialise{template: template.clone(), extract: self.extract.clone() }));
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
				save: true,
				extract: Vec::new()
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
						"sprite".to_string() => ParameterExpression::Argument("sprite".to_string()),
						"height".to_string() => ParameterExpression::Constant(Parameter::Float(0.1)),
						"name".to_string() => ParameterExpression::Constant(Parameter::String("grass".to_string()))
					))
				],
				save: true,
				extract: Vec::new()
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
// 			assert_eq!(result, "not a valid componenttype");
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
// 		assert_eq!(result, "parameter type incorrect");
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
// 		assert_eq!(result, "unknown argument name");
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
// 		assert_eq!(result, "parameter type incorrect");
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
// 		assert_eq!(result, "invalid argument default");
	}
	
	
	#[test]
	fn null_argument(){
		let result = Assemblage::from_json(&json!({
				"arguments": [
					["sprite", "string"]
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
						"sprite".to_string() => ParameterExpression::Argument("sprite".to_string()),
						"height".to_string() => ParameterExpression::Constant(Parameter::Float(0.1)),
						"name".to_string() => ParameterExpression::Argument("sprite".to_string())
					))
				],
				save: true,
				extract: Vec::new()
			};
		assert_eq!(result, constructed);
	}
}
