

use std::collections::HashMap;
use crate::parameter::Parameter;

#[derive(Debug)]
pub struct Template {
	pub name: String,
	pub args: Vec<Parameter>,
	pub kwargs: HashMap<String, Parameter>
}


impl Template {
	
	pub fn new(name: &str, kwargs: HashMap<String, Parameter>) -> Self {
		Self {
			name: name.to_string(),
			args: Vec::new(),
			kwargs
		}
	}
	
	pub fn empty(name: &str) -> Self {
		Self::new(name, HashMap::new())
	}
}
