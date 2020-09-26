
use std::collections::HashMap;
use serde::{de, Serialize, Deserialize, Deserializer};
use crate::{
	parameterexpression::ParameterExpression,
	parameter::{Parameter, ParameterType},
	componentwrapper::{ComponentWrapper, ComponentType},
	components::{Serialise, Clan},
	Template,
	Result as AnyResult,
	aerr
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
	
	pub fn validate(&self) -> AnyResult<()> {
		for (comptype, parameters) in &self.components {
			for paramname in comptype.parameters() {
				let _param = parameters.get(paramname).ok_or(aerr!("missing parameter {} for component {:?}", paramname, comptype))?;
				// todo: validate parameter types
			}
		}
		Ok(())
	}
	
	fn prepare_arguments(&self, args: &[Parameter], kwargs: &HashMap<String, Parameter>) -> AnyResult<HashMap<&str, Parameter>> {
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

	pub fn instantiate(&self, template: &Template) -> AnyResult<Vec<ComponentWrapper>>{
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
		if let Some(clan) = &template.clan {
			components.push(ComponentWrapper::Clan(Clan{name: clan.clone()}));
		}
		Ok(components)
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
			arguments: arguments.into_iter()
				.map(|arg| match arg {
					ArgumentDefSave::Long(name, typ, def) => (name, typ, Some(def)),
					ArgumentDefSave::Short(name, typ) => (name, typ, None)
				})
				.collect(),
			components,
			save,
			extract: extract.into_iter().map(|(k, (t, v))| (k, t, v)).collect()
		})
	}
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum ArgumentDefSave{
	Long(String, ParameterType, Parameter),
	Short(String, ParameterType)
}
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct AssemblageSave {
	#[serde(default)]
	pub arguments: Vec<ArgumentDefSave>,
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
	fn grass_deserialize(){
		let result = Assemblage::deserialize(&json!({
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
		Assemblage::deserialize(&json!({
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
	
	
	
// 	#[test]
	fn invalid_parameter_type(){
		Assemblage::deserialize(&json!({
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
	
// 	#[test]
	fn unknown_argument_name(){
		Assemblage::deserialize(&json!({
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
	
// 	#[test]
	fn wrong_argument_type(){
		Assemblage::deserialize(&json!({
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
	
	
	
// 	#[test]
	fn wrong_argument_default(){
		Assemblage::deserialize(&json!({
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
		let result = Assemblage::deserialize(&json!({
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
