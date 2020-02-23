
use std::collections::HashMap;
use serde_json::Value;
use crate::{
	assemblage::Assemblage,
	componentwrapper::PreEntity,
	Template
};

#[derive(Default, Clone)]
pub struct Encyclopedia {
	items: HashMap<String, Assemblage>
}

impl Encyclopedia {
	
	pub fn from_json(val: Value) -> Result<Encyclopedia, &'static str> {
		let mut items = HashMap::new();
		for (k, v) in val.as_object().ok_or("encyclopedia not a json object")?.into_iter() {
			items.insert(k.clone(), Assemblage::from_json(v)?);
		}
		Ok(Encyclopedia{items})
	}
	
	pub fn construct(&self, template: &Template) -> Result<PreEntity, &'static str> {
		let assemblage = self.items.get(&template.name).ok_or("unknown assemblage name")?;
		assemblage.instantiate(template)
	}
	
}

