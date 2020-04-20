
use std::collections::HashMap;
use serde_json::{json, Value};
use crate::{
	Pos,
	Template,
	PResult,
	perr
};

#[derive(Debug, Clone)]
pub struct RoomTemplate {
	pub size: (i64, i64),
	pub spawn: Pos,
	pub field: Vec<Vec<Template>>,
	pub places: HashMap<String, Pos>
}

impl RoomTemplate {

	pub fn from_json(jsonroom: &Value) -> PResult<RoomTemplate>{
		let size = (
			jsonroom.get("width").ok_or(perr!("no with"))?.as_i64().ok_or(perr!("with not a number"))?,
			jsonroom.get("height").ok_or(perr!("no height"))?.as_i64().ok_or(perr!("height not a number"))?
		);
		let spawn = Pos::from_json(jsonroom.get("spawn").ok_or(perr!("no spawn"))?).ok_or(perr!("spawn not a pos"))?;
		
		let mut mapping = HashMap::new();
		for (key, value) in jsonroom.get("mapping").ok_or(perr!("no mapping"))?.as_object().ok_or(perr!("mapping not a json object"))?.iter() {
			let mut templates: Vec<Template> = Vec::new();
			if value.is_array() {
				for template in value.as_array().unwrap() {
					templates.push(Template::from_json(template)?);
				}
			} else {
				templates.push(Template::from_json(value)?);
			}
			mapping.insert(key.chars().next().ok_or(perr!("mapping key is empty string"))?, templates);
		}
		
		let width = size.0 as usize;
		let height = size.1 as usize;
		let mut field = Vec::new();
		field.resize_with(width * height, Vec::new);
		let jsonfield: &Vec<Value> = jsonroom.get("field").ok_or(perr!("no field"))?.as_array().ok_or(perr!("field not an array"))?;
		for (y, row) in jsonfield.iter().take(height).enumerate() {
			for (x, ch) in row.as_str().ok_or(perr!("field row not a string"))?.chars().take(width).enumerate() {
				field[x + y * width] = mapping.get(&ch).ok_or(perr!("char not found in mapping"))?.clone();
			}
		}
		
		let mut places = HashMap::new();
		for (name, jsonpos) in jsonroom.get("places").unwrap_or(&json!({})).as_object().ok_or(perr!("places not an object"))? {
			places.insert(name.to_string(), Pos::from_json(jsonpos).ok_or(perr!("pos of places invalid"))?);
		}
		
		Ok(RoomTemplate {
			size,
			spawn,
			field,
			places
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	#[test]
	fn simple_from_json() {
		RoomTemplate::from_json(&json!({
			"width": 6,
			"height": 5,
			"spawn": [1, 1],
			"field": [
				"######",
				"#,,,,#",
				"#,,,,#",
				"#....#",
				"######"
			],
			"mapping": {
				"#": ["wall"],
				",": "grass",
				".": {"type": "grass", "args": [], "kwargs": {}}
			}
		})).unwrap();
	}
}
