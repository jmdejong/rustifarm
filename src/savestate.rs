
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

use crate::{
	Pos,
	Template,
};


#[derive(Debug, Clone, PartialEq)]
pub struct SaveState {
	pub changes: HashMap<Pos, Vec<Template>>
}

impl SaveState {
	
	pub fn new() -> Self {
		Self {
			changes: HashMap::new()
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct SaveStateVec {pub changes: Vec<(Pos, Vec<Template>)>}

impl Serialize for SaveState {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer {
		SaveStateVec{changes: self.changes.clone().into_iter().collect()}.serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for SaveState {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		Ok(Self{changes: SaveStateVec::deserialize(deserializer)?.changes.into_iter().collect()})
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use crate::hashmap;
	use serde_json::json;
	
	#[test]
	fn test_empty_deserialize(){
		assert_eq!(SaveState::deserialize(json!({"changes":[]})).unwrap(), SaveState::new());
	}
	
	#[test]
	fn test_empty_serialize(){
		assert_eq!(serde_json::to_value(SaveState::new()).unwrap(), json!({"changes":[]}));
	}
	
	
	#[test]
	fn test_change_serialize(){
		assert_eq!(
			serde_json::to_value(
				SaveState{changes: hashmap!{Pos::new(5,2) => vec![Template::empty("grass"), Template::empty("tree")]}}
			).unwrap(),
			json!({"changes":[[[5,2],["grass", "tree"]]]})
		);
	}
	
	#[test]
	fn test_changes_deserialize(){
		assert_eq!(
			SaveState::deserialize(json!({"changes":[[[1,1],["grass"]], [[5,2],["grass", "tree"]]]})).unwrap(),
			SaveState{changes: hashmap!{
				Pos::new(1,1) => vec![Template::empty("grass")],
				Pos::new(5,2) => vec![Template::empty("grass"), Template::empty("tree")]
			}}
		);
	}
}
