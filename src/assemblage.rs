
use std::collections::HashMap;
use serde_json::{Value, json};
use crate::{
	componentparameter::ComponentParameter,
	parameter::{Parameter, ParameterType},
	componentwrapper::{ComponentWrapper, ComponentType},
	components::Serialise,
	hashmap,
	Template,
	Result,
	aerr
};

type ArgumentDef = (String, ParameterType, Option<Parameter>);

#[derive(Debug, PartialEq, Clone)]
pub struct Assemblage {
	pub arguments: Vec<ArgumentDef>,
	pub components: Vec<(ComponentType, HashMap<String, ComponentParameter>)>,
	pub save: bool,
	pub extract: Vec<(String, ComponentType, String)>
}

impl Assemblage {


	fn parse_definition_arguments(args: &Value) -> Result<Vec<ArgumentDef>> {
		let mut arguments: Vec<ArgumentDef> = Vec::new();
		for arg in args.as_array().ok_or(aerr!("arguments is not an array"))? {
			let tup = arg.as_array().ok_or(aerr!("argument is not an array"))?;
			let key = tup.get(0).ok_or(aerr!("argument has no name"))?.as_str().ok_or(aerr!("argument name is not a string"))?.to_string();
			let typ = ParameterType::from_str(tup.get(1).ok_or(aerr!("argument has no type"))?.as_str().ok_or(aerr!("argument type not a string"))?).ok_or(aerr!("failed to parse argument type"))?;
			if let Some(def) = tup.get(2){
				arguments.push(
					(
						key.clone(),
						typ,
						Some(Parameter::from_typed_json(typ, def).ok_or(aerr!("invalid argument default"))?)
					)
				);
			} else  {
				arguments.push((key.clone(), typ, None));
			}
		}
		Ok(arguments)
	}
	
	fn parse_definition_components(comps: &Value) -> Result<Vec<(ComponentType, HashMap<String, ComponentParameter>)>> {
		let mut components = Vec::new();
		for tup in comps.as_array().ok_or(aerr!("components is not a json array"))? {
			if let Some(name) = tup.as_str() {
				components.push((ComponentType::from_str(name).ok_or(aerr!("not a valid componenttype"))?, HashMap::new()));
			} else {
				let comptype = ComponentType::from_str(tup
					.get(0).ok_or(aerr!("index 0 not in component"))?
					.as_str().ok_or(aerr!("component name not a string"))?
				).ok_or("not a valid componenttype")?;
				let mut parameters: HashMap<String, ComponentParameter> = HashMap::new();
				for (key, value) in tup.get(1).ok_or(aerr!("index 1 not in component"))?.as_object().ok_or(aerr!("component parameters not a json object"))? {
					let param = ComponentParameter::from_json(value)?;
					parameters.insert(key.clone(), param);
				}
				components.push((comptype, parameters));
			}
		}
		Ok(components)
	}
	
	fn validate(&self) -> Result<()> {
		for (comptype, parameters) in &self.components {
			for (paramname, paramtype) in comptype.parameters() {
				let param = parameters.get(paramname).ok_or(aerr!("missing parameter"))?;
				let actualtype = param.get_type(&self.arguments)?;
				if actualtype != paramtype {
					return Err(aerr!("parameter type incorrect"));
				}
			}
		}
		Ok(())
	}
	
	fn common_short_definitions(val: &Value) -> Result<Vec<(ComponentType, HashMap<String, ComponentParameter>)>> {
		let mut components = Vec::new();
		
		let name = if let Some(nameval) = val.get("name") {
				Some(nameval.as_str().ok_or(aerr!("name not a string"))?.to_string())
			} else {None};
		
		// visible component is so common that shortcuts are very helpful
		if let Some(spritename) = val.get("sprite") {
			let sprite = spritename.as_str().ok_or(aerr!("sprite not a string"))?.to_string();
			let height = val
				.get("height").ok_or(aerr!("defining a sprite requires also defining a height"))?
				.as_f64().ok_or(aerr!("height not a float"))?;
			components.push((
				ComponentType::Visible,
				hashmap!(
					"name".to_string() => ComponentParameter::Constant(
						Parameter::String(name.clone().unwrap_or(sprite.clone()))
					),
					"sprite".to_string() => ComponentParameter::Constant(
						Parameter::String(sprite)
					),
					"height".to_string() => ComponentParameter::Constant(
						Parameter::Float(height)
					)
				)
			));
		}
		
		// item component is common too
		if let Some(action) = val.get("item") {
			components.push((
				ComponentType::Item,
				hashmap!(
					"ent".to_string() => ComponentParameter::TemplateSelf,
					"name".to_string() => if let Some(n) = name {
							ComponentParameter::Constant(Parameter::String(n))
						} else {
							ComponentParameter::TemplateName
						},
					"action".to_string() => ComponentParameter::Constant(
						Parameter::from_typed_json(ParameterType::Action, action).ok_or(aerr!("invalid item action"))?
					)
				)
			));
		}
		
		// and so is flags
		if let Some(flags) = val.get("flags") {
			components.push((
				ComponentType::Flags,
				hashmap!(
					"flags".to_string() => ComponentParameter::Constant(
						Parameter::from_typed_json(ParameterType::List, flags).ok_or(aerr!("failed to parse flags"))?
					)
				)
			));
		}
		
		Ok(components)
	}
	
	pub fn from_json(val: &Value) -> Result<Self>{
		let mut assemblage = Self {
			arguments: Self::parse_definition_arguments(val.get("arguments").unwrap_or(&json!([])))?,
			components: Self::parse_definition_components(val.get("components").unwrap_or(&json!([])))?,
			save: val.get("save").unwrap_or(&json!(true)).as_bool().ok_or(aerr!("assemblage save not a bool"))?,
			extract: val
				.get("extract")
				.unwrap_or(&json!({}))
				.as_object().ok_or(aerr!("assemblage extract not a bool"))?
				.into_iter()
				.map(|(argname, val)| {
					Ok((
						argname.to_string(),
						ComponentType::from_str(
							val
								.get(0).ok_or(aerr!("index 0 not in extract value"))?
								.as_str().ok_or(aerr!("extract component name not a string"))?
						).ok_or(aerr!("extract invalid component name"))?,
						val.get(1)
							.ok_or(aerr!("index 1 not in extract value"))?
							.as_str().ok_or(aerr!("extract member name not a string"))?.to_string()
					))
				})
				.collect::<Result<Vec<(String, ComponentType, String)>>>()?
		};
		assemblage.components.append(&mut Self::common_short_definitions(val)?);
		assemblage.validate()?;
		Ok(assemblage)
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
			let param = value.ok_or(aerr!(&format!("argument <{:?}> has no value", (idx, (name, typ, def)))))?;
			if param.paramtype() != *typ {
				return Err(aerr!(&format!(
					"argument has incorrect type: {:?}, {:?}, {:?}",
					(idx, (name, typ, def)),
					param.paramtype(),
					param
				)));
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
		if template.save && self.save {
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
						"sprite".to_string() => ComponentParameter::Argument("sprite".to_string()),
						"height".to_string() => ComponentParameter::Constant(Parameter::Float(0.1)),
						"name".to_string() => ComponentParameter::Constant(Parameter::String("grass".to_string()))
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
						"sprite".to_string() => ComponentParameter::Argument("sprite".to_string()),
						"height".to_string() => ComponentParameter::Constant(Parameter::Float(0.1)),
						"name".to_string() => ComponentParameter::Argument("sprite".to_string())
					))
				],
				save: true,
				extract: Vec::new()
			};
		assert_eq!(result, constructed);
	}
}
