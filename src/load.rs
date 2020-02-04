
use std::collections::HashMap;
use serde_json::Value;
use super::assemblage::Assemblage;
use super::assemblages::{Player, Grass, Wall};

pub fn load_assemblages(data: Value) -> Vec<Box<dyn Assemblage>> {
	parse_assemblages(data).into_iter().filter_map(|x| x).collect()
}

fn parse_assemblages(data: Value) -> Vec<Option<Box<dyn Assemblage>>> {
	match data {
		Value::String(txt) => vec![from_args(txt, Vec::new(), HashMap::new())],
		Value::Array(list) => list.into_iter().map(parse_assemblages).flatten().collect(),
		Value::Object(mut obj) => {
			if let Some(Value::String(typename)) = obj.remove("type") {
				let args = if let Some(Value::Array(a)) = obj.remove("args") {a} else {Vec::new()};
				let kwargs: HashMap<String, Value> = if let Some(Value::Object(o)) = obj.remove("kwargs") {
					o.into_iter().collect()
				} else {HashMap::new()};
				vec![from_args(
					typename,
					args,
					kwargs
				)]
			} else {Vec::new()}
		},
		_ => Vec::new()
	}
}

macro_rules! dynasm {
	($typ:ident) => {Some({
		let b : Box<dyn Assemblage> = Box::new({
			let o = $typ::default();
			o
		});
		b
	})}
}

fn from_args(typename: String, args: Vec<Value>, kwargs: HashMap<String, Value>) -> Option<Box<dyn Assemblage>>{
	 let mut obj = match typename.as_str() {
		"player" => dynasm!(Player),
		"grass" => dynasm!(Grass),
		"wall" => dynasm!(Wall),
		_ => None
	}?;
	obj.init_from_json(args, kwargs);
	Some(obj)
}


#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	use std::any::Any;
	#[test]
	fn test_assemblage_from_json() {
		let mut walls1 = load_assemblages(json!("wall"));
		assert_eq!(walls1.len(), 1);
		let wallbox1 = walls1.pop().unwrap();
// 		assert_eq!(, vec![Box::new(Wall{})]);
// 		assert_eq!(load_assemblages(json!("wall", ["test"], {"abc": 123})), Wall{});
	}
}


