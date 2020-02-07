

use std::collections::HashMap;
use crate::parameter::Parameter;

#[derive(Debug)]
pub struct Template {
	pub name: String,
	pub args: Vec<Parameter>,
	pub kwargs: HashMap<String, Parameter>
}


impl Template {
	
	pub fn empty(name: &str) -> Self {
		Self {
			name: name.to_string(),
			args: Vec::new(),
			kwargs: HashMap::new()
		}
	}
}
