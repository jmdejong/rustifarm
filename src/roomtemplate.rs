
use std::collections::HashMap;
use serde_json::Value;
use crate::Pos;
use crate::template::Template;

pub struct RoomTemplate {
	pub size: (i64, i64),
	pub spawn: Pos,
	pub field: Vec<Vec<Template>>
}

impl RoomTemplate {
	
	pub fn from_json(jsonroom: &Value) -> Result<RoomTemplate, &'static str>{
		let size = (
			jsonroom.get("width").ok_or("no with")?.as_i64().ok_or("with not a number")?,
			jsonroom.get("height").ok_or("no height")?.as_i64().ok_or("height not a number")?
		);
		let spawn = Pos::from_json(jsonroom.get("spawn").ok_or("no spawn")?).ok_or("spawn not a pos")?;
		
		let mut mapping = HashMap::new();
		for (key, value) in jsonroom.get("mapping").ok_or("no mapping")?.as_object().ok_or("mapping not a json object")?.iter() {
			let mut templates: Vec<Template> = Vec::new();
			if value.is_array() {
				for template in value.as_array().ok_or("weird")? {
					templates.push(Template::from_json(template).ok_or("not a valid template")?);
				}
			} else {
				templates.push(Template::from_json(value).ok_or("not a valid template")?);
			}
			mapping.insert(key.chars().next().ok_or("mapping key is empty string")?, templates);
		}
		
		let mut field = Vec::new();
		field.resize((size.0 * size.1) as usize, Vec::new());
		let jsonfield: &Vec<Value> = jsonroom.get("field").ok_or("no field")?.as_array().ok_or("field not an array")?;
		for (y, row) in jsonfield.iter().enumerate() {
			for (x, ch) in row.as_str().ok_or("field row not a string")?.chars().enumerate() {
				field[x + y * (size.0 as usize)] = mapping.get(&ch).ok_or("char not found in mapping")?.clone();
			}
		}
		Ok(RoomTemplate {
			size,
			spawn,
			field
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
