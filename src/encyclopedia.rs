
use std::collections::HashMap;
use serde_json::Value;
use crate::assemblage::Assemblage;
use crate::componentwrapper::ComponentWrapper;
use crate::template::Template;

#[derive(Default, Clone)]
pub struct Encyclopedia {
	items: HashMap<String, Assemblage>
}

impl Encyclopedia {
	
	pub fn new() -> Encyclopedia {
		Encyclopedia {
			items: HashMap::new()
		}
	}
	
	pub fn from_json(val: Value) -> Result<Encyclopedia, &'static str> {
		let mut items = HashMap::new();
		for (k, v) in val.as_object().ok_or("encyclopedia not a json object")?.into_iter() {
			items.insert(k.clone(), Assemblage::from_json(v)?);
		}
		Ok(Encyclopedia{items})
	}
	
	pub fn add_assemblage(&mut self, name: &str, assemblage: Assemblage) -> Result<(), &'static str> {
		//todo: what if name exists
		self.items.insert(name.to_string(), assemblage);
		Ok(())
	}
	
	pub fn construct(&self, template: &Template) -> Result<Vec<ComponentWrapper>, &'static str> {
		let assemblage = self.items.get(&template.name).ok_or("unknown assemblage name")?;
		assemblage.instantiate(&template.args, &template.kwargs)
	}
	
}

