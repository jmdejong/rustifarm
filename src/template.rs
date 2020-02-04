
use std::collections::HashMap;
use serde_json::Value;
use super::compwrapper::CompWrapper;

pub struct Template {
	pub arguments: Vec<String>,
	pub components: Vec<(String, HashMap<String, CompParam>)>
}

impl Template {
	pub fn instantiate(&self, args: Vec<Value>, kwargs: HashMap<String, Value>) -> Option<Vec<CompWrapper>>{
		let mut components: Vec<CompWrapper> = Vec::new();
		for (compname, compparams) in &self.components {
			let mut compargs: HashMap<&str, &Value> = HashMap::new();
			for (name, param) in compparams {
				match param {
					CompParam::Constant(val) => {compargs.insert(name.as_str(), &val); Some(())},
					CompParam::Argument(argname) => {
						if let Some(argval) = kwargs.get(argname.as_str()) {
							compargs.insert(name.as_str(), argval);
							Some(())
						} else if let Some(idx) = self.arguments.iter().position(|x| x == name){
							if idx < args.len() {
								compargs.insert(name.as_str(), &args[idx]);
								Some(())
							} else {
								println!("positional argument out of range");
								None
							}
						} else {
							println!("can't find parameter value, compname: {}, name: {}, argname: {}", compname, name, argname);
							None
						}
					}
				}?;
			}
			components.push(CompWrapper::load_component(compname.as_str(), compargs)?);
		}
		Some(components)
	}
}


pub enum CompParam {
	Constant(Value),
	Argument(String)
}
