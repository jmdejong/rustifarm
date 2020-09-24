
use std::collections::HashMap;
use serde::{Deserialize, Deserializer, de, Serialize};
use crate::{
	Pos,
	Template,
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
