
use std::collections::HashMap;
use serde_json::Value;
use crate::{
	assemblage::Assemblage,
	componentwrapper::PreEntity,
	Template,
	template::EntityType,
	Result,
	aerr
};

#[derive(Default, Clone)]
pub struct Encyclopedia {
	items: HashMap<EntityType, Assemblage>
}

impl Encyclopedia {
	
	pub fn from_json(val: Value) -> Result<Encyclopedia> {
		let mut items = HashMap::new();
		for (k, v) in val.get("assemblages").ok_or(aerr!("no assemblages in encyclopedia json"))?.as_object().ok_or(aerr!("encyclopedia not a json object"))?.into_iter() {
			items.insert(EntityType(k.clone()), Assemblage::from_json(v)?);
		}
		Ok(Encyclopedia{items})
	}
	
	pub fn construct(&self, template: &Template) -> Result<PreEntity> {
		let assemblage = self.items.get(&template.name).ok_or(aerr!("unknown assemblage name"))?;
		assemblage.instantiate(template)
	}
	
}

