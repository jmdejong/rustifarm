
use std::collections::HashMap;
use serde_json::{json, Value, value};
use serde::{Deserialize, Deserializer, de, Serialize};
use crate::{
	Pos,
	Template,
	PResult,
	perr,
	resources::RoomPermissions
};

#[derive(Debug, Clone)]
pub struct RoomTemplate {
	pub size: (i64, i64),
	pub spawn: Pos,
	pub field: Vec<Vec<Template>>,
	pub places: HashMap<String, Pos>,
	pub permissions: RoomPermissions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoomTemplateSave {
	pub width: i64,
	pub height: i64,
	pub spawn: Pos,
	pub field: Vec<String>,
	pub mapping: HashMap<char, TemplateList>,
	#[serde(default)]
	pub places: HashMap<String, Pos>,
	#[serde(default)]
	pub permissions: RoomPermissions
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum TemplateList {
	Single(Template),
	List(Vec<Template>)
}
impl<'de> Deserialize<'de> for RoomTemplate {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de>,
	{
		let rts = RoomTemplateSave::deserialize(deserializer)?;
		let mut field = Vec::new();
		let width = rts.width as usize;
		let height = rts.height as usize;
		field.resize_with(width * height, Vec::new);
		for (y, row) in rts.field.iter().take(height).enumerate() {
			for (x, ch) in row.chars().take(width).enumerate() {
				let templates = rts.mapping.get(&ch).ok_or(de::Error::custom(format!("char {}not found in mapping", ch)))?.clone();
				field[x + y * width] = match templates {
					TemplateList::Single(temp) => vec![temp],
					TemplateList::List(temps) => temps
				}
			}
		}
		Ok(Self {
			size: (rts.width, rts.height),
			spawn: rts.spawn,
			field,
			places: rts.places,
			permissions: rts.permissions
		})
	}
}

impl RoomTemplate {

	pub fn from_json(jsonroom: &Value) -> PResult<RoomTemplate>{
		let size = (
			jsonroom.get("width").ok_or(perr!("no width"))?.as_i64().ok_or(perr!("width not a number"))?,
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
		
		let permissions: RoomPermissions = value::from_value::<RoomPermissions>(
			jsonroom
				.get("permissions")
				.unwrap_or(&json!({}))
				.clone()
			).map_err(|e| perr!("can't deserialise permissions: {:?}", e))?;
		
		Ok(RoomTemplate {
			size,
			spawn,
			field,
			places,
			permissions
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
	
	#[test]
	fn simple_from_json() {
		RoomTemplate::deserialize(&json!({
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
