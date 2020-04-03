
use serde_json::json;
use specs::{
	DispatcherBuilder
};

use crate::{
	RoomId,
	Encyclopedia,
	room::Room,
	roomtemplate::RoomTemplate,
	systems::{
		Move,
		RegisterNew,
		ControlInput,
		View,
		Remove,
		Create,
		Volate,
		UpdateCooldowns,
		ControlAI,
	}
};

pub fn purgatory_id() -> RoomId {
	RoomId{name: String::from("+")}
}

pub fn create_purgatory<'a, 'b>(encyclopedia: &Encyclopedia) -> Room<'a, 'b> {
	let dispatcher = DispatcherBuilder::new()
		.with(Volate, "volate", &[])
		.with(RegisterNew::default(), "registernew", &[])
		.with(UpdateCooldowns, "cool_down", &["registernew"])
		.with(ControlInput, "controlinput", &["cool_down"])
		.with(ControlAI, "controlai", &["cool_down"])
		.with(Move, "move", &["controlinput", "controlai"])
		.with(View::default(), "view", &["move", "volate"])
		.with(Create, "create", &["view"])
		.with(Remove, "remove", &["view", "move"])
		.build();
	let mut room = Room::new(purgatory_id(), encyclopedia.clone(), dispatcher);
	room.load_from_template(&RoomTemplate::from_json(&json!({
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
			"A": [{"type": "letter", "args": ["A"]}],
			"B": [{"type": "letter", "args": ["B"]}],
			"C": [{"type": "letter", "args": ["C"]}],
			"D": [{"type": "letter", "args": ["D"]}],
			"E": [{"type": "letter", "args": ["E"]}],
			"F": [{"type": "letter", "args": ["F"]}],
			"G": [{"type": "letter", "args": ["G"]}],
			"H": [{"type": "letter", "args": ["H"]}],
			"I": [{"type": "letter", "args": ["I"]}],
			"J": [{"type": "letter", "args": ["J"]}],
			"K": [{"type": "letter", "args": ["K"]}],
			"L": [{"type": "letter", "args": ["L"]}],
			"M": [{"type": "letter", "args": ["M"]}],
			"N": [{"type": "letter", "args": ["N"]}],
			"O": [{"type": "letter", "args": ["O"]}],
			"P": [{"type": "letter", "args": ["P"]}],
			"Q": [{"type": "letter", "args": ["Q"]}],
			"R": [{"type": "letter", "args": ["R"]}],
			"S": [{"type": "letter", "args": ["S"]}],
			"T": [{"type": "letter", "args": ["T"]}],
			"U": [{"type": "letter", "args": ["U"]}],
			"V": [{"type": "letter", "args": ["V"]}],
			"W": [{"type": "letter", "args": ["W"]}],
			"X": [{"type": "letter", "args": ["X"]}],
			"Y": [{"type": "letter", "args": ["Y"]}],
			"Z": [{"type": "letter", "args": ["Z"]}]
		}
	})).unwrap());
	room
}
