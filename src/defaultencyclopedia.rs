
use serde_json::json;
use crate::Encyclopedia;

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
				["Item", {
					"ent": ["template", "pebble"],
					"name": ["string", "pebble"],
					"action": ["action", ["eat", 1]]
				}]
			],
			"sprite": "pebble",
			"height": 0.3
		},
		"stone": {
			"components": [
				["Item", {"ent": ["template", "stone"], "name": ["string", "stone"], "action": ["action", ["build", "builtwall"]]}]
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
			"arguments": [["destination", "string", null], ["dest_pos", "string", ""]],
			"components": [
				["RoomExit", {"destination": ["arg", "destination"], "dest_pos": ["arg", "dest_pos"]}],
				"Floor"
			]
		},
		"builtwall": {
			"arguments": [["health", "int", 100]],
			"components": [
				"Blocking",
				["Health", {"health": ["arg", "health"], "maxhealth": ["int", 100]}]
			],
			"sprite": "wall",
			"height": 2
		},
		"spiketrap": {
			"components": [["Trap", {"damage": ["int", 8]}]],
			"sprite": "spikes",
			"height": 0.8
		},
		"dummy": {
			"arguments": [["health", "int", 20]],
			"sprite": "dummy",
			"height": 1,
			"components": [["Health", {"health": ["arg", "health"], "maxhealth": ["int", 20]}]]
		},
		"wound": {
			"sprite": "wound",
			"height": 0.25,
			"components": [["Volatile", {"delay": ["int", 4]}]]
		},
		"rat": {
			"sprite": "rat",
			"height": 1,
			"components": [
				["MonsterAI", {
					"view_distance": ["int", 3],
					"move_chance": ["float", 0.08],
					"homesickness": ["float", 0.1]
				}],
				["Health", {"health": ["int", 8], "maxhealth": ["int", 8]}],
				["Fighter", {"damage": ["int", 2], "cooldown": ["int", 6]}],
				["Movable", {"cooldown": ["int", 3]}]
			]
		}
	})).unwrap()
}
