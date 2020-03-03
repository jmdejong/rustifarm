
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

pub fn create_purgatory<'a, 'b>(encyclopedia: Encyclopedia) -> Room<'a, 'b> {
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
	let mut room = Room::new(purgatory_id(), encyclopedia, dispatcher);
	room.load_from_template(&RoomTemplate::from_json(&json!({
		"width": 11,
		"height": 11,
		"spawn": [5, 5],
		"field": [
			"    +++    ",
			"  +++++++  ",
			" +++++++++ ",
			" +++++++++ ",
			"+++++++++++",
			"+++++++++++",
			"+++++++++++",
			" +++++++++ ",
			" +++++++++ ",
			"  +++++++  ",
			"    +++    ",
		],
		"mapping": {
			" ": [],
			"+": ["floor"]
		}
	})).unwrap());
	room
}
