
use std::collections::HashMap;
use serde::{de, Serialize, Deserialize, Deserializer};
use crate::{
	parameterexpression::{ParameterExpression, EvaluationError},
	parameter::{Parameter},
	componentwrapper::{ComponentWrapper, ComponentType},
	components::{Serialise, Clan},
	Template,
	Result as AnyResult,
	aerr,
	sprite::Sprite,
	compmap
};


#[derive(Debug, PartialEq, Clone)]
pub struct Assemblage {
	arguments: HashMap<String, Option<Parameter>>,
	components: Vec<(ComponentType, HashMap<String, ParameterExpression>)>,
	save: bool,
	extract: Vec<(String, ComponentType, String)>
}

impl Assemblage {
	
	pub fn validate(&self) -> AnyResult<()> {
		
		for (comptype, parameters) in &self.components {
			let mut is_complete = true;
			let mut compargs = HashMap::new();
			for paramname in comptype.parameters() {
				let param = parameters.get(paramname).ok_or(aerr!("missing parameter {} for component {:?}", paramname, comptype))?;
				match param.evaluate(&self.arguments, &Template::empty("")) {
					Err(EvaluationError::MissingArgument(_)) => {is_complete = false;}
					Err(EvaluationError::Other(msg)) => {return Err(aerr!("invalid value for {}: {}", paramname, msg))}
					Ok(p) => {compargs.insert(paramname, p);}
				}
			}
			if is_complete {
				ComponentWrapper::load_component(*comptype, compargs)?;
			}
		}
		Ok(())
	}
	

	pub fn instantiate(&self, template: &Template) -> AnyResult<Vec<ComponentWrapper>>{
		let mut arguments = self.arguments.clone();
		for (key, param) in template.kwargs.clone() {
			arguments.insert(key, Some(param));
		}
		let mut components: Vec<ComponentWrapper> = Vec::new();
		for (comptype, compparams) in &self.components {
			let mut compargs: HashMap<&str, Parameter> = HashMap::new();
			for (name, param) in compparams {
				compargs.insert(name.as_str(), param.evaluate(&arguments, template).map_err(|e| match e {
					EvaluationError::MissingArgument(arg) => aerr!("argument {} has no value", arg),
					EvaluationError::Other(msg) => aerr!("{}", msg)
				})?);
			}
			components.push(ComponentWrapper::load_component(*comptype, compargs)?);
		}
		if template.should_save() && self.save {
			components.push(ComponentWrapper::Serialise(Serialise{template: template.clone(), extract: self.extract.clone() }));
		}
		if let Some(clan) = &template.clan {
			components.push(ComponentWrapper::Clan(Clan{name: clan.clone()}));
		}
		Ok(components)
	}
	
	pub fn apply_arguments(&self, arguments: HashMap<String, Parameter>) -> Self {
		let mut assemblage = self.clone();
		for (key, val) in arguments {
			assemblage.arguments.insert(key, Some(val));
		}
		assemblage
	}
	
	pub fn new_item(id: String, sprite: Sprite, name: String) -> Assemblage {
		Assemblage {
			arguments: HashMap::new(),
			save: true,
			extract: Vec::new(),
			components: vec![
				(ComponentType::Visible, compmap!{height: 0.3_f64, sprite: sprite.0, name: name}),
				(ComponentType::Item, compmap!{item: id})
			]
		}
	}
}

#[macro_export]
macro_rules! compmap {
	{$($name: ident: $val: expr),*} => {{
		#[allow(unused_imports)]
		use crate::fromtoparameter::FromToParameter;
		#[allow(unused_mut)]
		let mut h = std::collections::HashMap::new();
		$(
			h.insert(stringify!($name).to_string(), crate::parameterexpression::ParameterExpression::Constant($val.to_parameter()));
		)*
		h
	}}
}

impl<'de> Deserialize<'de> for Assemblage {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let AssemblageSave{arguments, mut components, save, extract, name, sprite, height, flags, substitute} =
			AssemblageSave::deserialize(deserializer)?;
		if let Some(f) = flags {
			components.push((ComponentType::Flags, compmap!{flags: f}));
		}
		if let Some(spr) = sprite {
			components.push((ComponentType::Visible, compmap!{
				sprite: spr.clone(),
				height: height.ok_or(de::Error::custom("height must be included in assemblage when sprite is included"))?,
				name: name.unwrap_or(spr)
			}));
		}
		if let Some(sub) = substitute {
			components.push((ComponentType::Substitute, compmap!{into: sub}));
		}
		Ok(Assemblage {
			arguments,
			components,
			save,
			extract: extract.into_iter().map(|(k, (t, v))| (k, t, v)).collect()
		})
	}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AssemblageSave {
	#[serde(default)]
	pub arguments: HashMap<String, Option<Parameter>>,
	#[serde(default)]
	pub components: Vec<(ComponentType, HashMap<String, ParameterExpression>)>,
	#[serde(default="return_true")]
	pub save: bool,
	#[serde(default)]
	pub extract: HashMap<String, (ComponentType, String)>,
	pub name: Option<String>,
	pub sprite: Option<String>,
	pub height: Option<f64>,
	pub flags: Option<Vec<String>>,
	pub substitute: Option<Template>
}
fn return_true() -> bool {true}



#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	use serde_json::json;
	
	
	#[test]
	fn empty_assemblage_deserialize() {
		assert_eq!(
				Assemblage::deserialize(&json!({
				"arguments": {},
				"components": []
			})).unwrap(),
			Assemblage{
				arguments: hashmap!{},
				components: vec![],
				save: true,
				extract: Vec::new()
			}
		);
	}
	
	#[test]
	fn grass_deserialize(){
		let result = Assemblage::deserialize(&json!({
				"arguments": {"sprite": "grass1"},
				"components": [
					["Visible", {
						"sprite": {"$arg": "sprite"},
						"height": 0.1,
						"name": "grass"
					}]
				]
			})).unwrap();
		let constructed = Assemblage{
				arguments: hashmap!{"sprite".to_string() => Some(Parameter::String("grass1".to_string()))},
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
		Assemblage::deserialize(&json!({
				"arguments": {"sprite": null},
				"components": [
					["visible", { // no capital so invalid
						"sprite": {"$arg": "sprite"},
						"height": 0.1,
						"name": "grass"
					}]
				]
			})).unwrap_err();
	}
	
	
	
	#[test]
	fn invalid_parameter_type(){
		Assemblage::deserialize(&json!({
				"arguments": {"sprite": "grass1"},
				"components": [
					["Visible", {
						"sprite": {"$arg": "sprite"},
						"height": "0.1",
						"name": "grass"
					}]
				]
			})).unwrap().validate().unwrap_err();
	}
	
	
	
	
	#[test]
	fn wrong_argument_default(){
		Assemblage::deserialize(&json!({
				"arguments": {"sprite": 1},
				"components": [
					["Visible", {
						"sprite": {"$arg": "sprite"},
						"height": 0.1,
						"name": "grass"
					}]
				]
			})).unwrap().validate().unwrap_err();
	}	
	
	#[test]
	fn missing_argument_default(){
		Assemblage::deserialize(&json!({
				"arguments": {"sprite": null},
				"components": [
					["Visible", {
						"sprite": {"$arg": "sprite"},
						"height": 0.1,
						"name": "grass"
					}]
				]
			})).unwrap().validate().unwrap();
	}
	
	
	#[test]
	fn unknown_argument(){
		Assemblage::deserialize(&json!({
				"arguments": {"name": "me"},
				"components": [
					["Visible", {
						"sprite": {"$arg": "sprite"},
						"height": 0.1,
						"name": "grass"
					}]
				]
			})).unwrap().validate().unwrap_err();
	}
	
	#[test]
	fn missing_component_parameter(){
		Assemblage::deserialize(&json!({
				"arguments": {},
				"components": [
					["Visible", {
						"height": 0.1,
						"name": "grass"
					}]
				]
			})).unwrap().validate().unwrap_err();
	}
	
	#[test]
	fn null_argument(){
		let result = Assemblage::deserialize(&json!({
				"arguments": {"sprite": null},
				"components": [
					["Visible", {
						"sprite": {"$arg": "sprite"},
						"height": 0.1,
						"name": {"$arg": "sprite"}
					}]
				]
			})).unwrap();
		let constructed = Assemblage{
				arguments: hashmap!{"sprite".to_string() => None},
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
