
use crate::encyclopedia::Encyclopedia;
use serde_json::json;

pub fn default_encyclopedia() -> Encyclopedia {
	Encyclopedia::from_json(json!({
		"wall": {
			"components": ["Blocking"],
			"sprite": "wall",
			"height": 2
		},
		"rock": {
			"components": ["Blocking"],
			"sprite": "rock",
			"height": 10
		},
		"tree": {
			"components": ["Blocking"],
			"sprite": "tree",
			"height": 3
		},
		"fence": {
			"components": ["Blocking"],
			"sprite": "fence",
			"height": 1
		},
		"grass": {
			"components": [
				["Visible", {
					"sprite": ["random", [
						["string", "grass1"],
						["string", "grass2"],
						["string", "grass3"],
						["string", "grass1"],
						["string", "grass2"],
						["string", "grass3"],
						["string", "ground"]
					]],
					"height": ["float", 0.1],
					"name": ["string", "grass"]
				}],
				"Floor"
			]
		},
		"greengrass": {
			"components": [
				["Visible", {
					"sprite": ["random", [
						["string", "grass1"],
						["string", "grass2"],
						["string", "grass3"]
					]],
					"height": ["float", 0.1],
					"name": ["string", "grass"]
				}],
				"Floor"
			]
		},
		"ground": {
			"components": ["Floor"],
			"sprite": "ground",
			"height": 0.1
		},
		"floor": {
			"components": ["Floor"],
			"sprite": "floor",
			"height": 0.1
		},
		"bridge": {
			"components": [
				"Floor"
			],
			"sprite": "bridge",
			"height": 0.1
		},
		"water": {
			"components": [],
			"sprite": "water",
			"height": 0.1
		},
		"pebble": {
			"components": [
				["Item", {"ent": ["template", "pebble"], "name": ["string", "pebble"]}]
			],
			"sprite": "pebble",
			"height": 0.3
		},
		"stone": {
			"components": [
				["Item", {"ent": ["template", "stone"], "name": ["string", "stone"]}]
			],
			"sprite": "stone",
			"height": 0.4
		},
		"player": {
			"arguments": [["name", "string", null]],
			"components": [
				["Visible", {
					"sprite": ["string", "player"],
					"height": ["float", 1.0],
					"name": ["arg", "name"]
				}],
				["Player", {
					"name": ["arg", "name"]
				}],
				["Inventory", {"capacity": ["int", 3]}],
				["Health", {"health": ["int", 9], "maxhealth": ["int", 10]}]
			]
		},
		"portal": {
			"arguments": [["destination", "string", null]],
			"components": [
				["RoomExit", {"destination": ["arg", "destination"]}],
				"Floor"
			]
		}
	})).unwrap()
}
