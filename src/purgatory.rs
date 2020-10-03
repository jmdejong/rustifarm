

use serde_json::json;
use serde::Deserialize;

use crate::{
	RoomId,
	Encyclopedia,
	room::{Room, RoomType},
	roomtemplate::RoomTemplate
};

pub fn purgatory_id() -> RoomId {
	RoomId(String::from("+"))
}

pub fn create_purgatory<'a, 'b>(encyclopedia: &Encyclopedia) -> Room {
	let mut room = Room::new(purgatory_id(), encyclopedia.clone(), RoomType::Purgatory);
	room.load_from_template(&RoomTemplate::deserialize(&json!({
		"width": 15,
		"height": 20,
		"spawn": [7, 9],
		"field": [
			" YOU HAVE DIED ",
			"               ",
			"               ",
			"      +++      ",
			"    +++++++    ",
			"   +++++++++   ",
			"   +++++++++   ",
			"  +++++++++++  ",
			"  +++++++++++  ",
			"  +++++++++++  ",
			"   +++++++++   ",
			"   +++++++++   ",
			"    +++++++    ",
			"      +++      ",
			"               ",
			"               ",
			" RESTART CLIENT",
			"   TO RESPAWN  "
		],
		"mapping": {
			" ": [],
			"+": ["floor"],
			"A": [{":template": "letter", "char": "A"}],
			"B": [{":template": "letter", "char": "B"}],
			"C": [{":template": "letter", "char": "C"}],
			"D": [{":template": "letter", "char": "D"}],
			"E": [{":template": "letter", "char": "E"}],
			"F": [{":template": "letter", "char": "F"}],
			"G": [{":template": "letter", "char": "G"}],
			"H": [{":template": "letter", "char": "H"}],
			"I": [{":template": "letter", "char": "I"}],
			"J": [{":template": "letter", "char": "J"}],
			"K": [{":template": "letter", "char": "K"}],
			"L": [{":template": "letter", "char": "L"}],
			"M": [{":template": "letter", "char": "M"}],
			"N": [{":template": "letter", "char": "N"}],
			"O": [{":template": "letter", "char": "O"}],
			"P": [{":template": "letter", "char": "P"}],
			"Q": [{":template": "letter", "char": "Q"}],
			"R": [{":template": "letter", "char": "R"}],
			"S": [{":template": "letter", "char": "S"}],
			"T": [{":template": "letter", "char": "T"}],
			"U": [{":template": "letter", "char": "U"}],
			"V": [{":template": "letter", "char": "V"}],
			"W": [{":template": "letter", "char": "W"}],
			"X": [{":template": "letter", "char": "X"}],
			"Y": [{":template": "letter", "char": "Y"}],
			"Z": [{":template": "letter", "char": "Z"}]
		}
	})).unwrap()).unwrap();
	room
}
