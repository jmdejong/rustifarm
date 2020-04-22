
# Asciifarm map format

The maps in asciifarm use a json format.
The main node is a json object.
This object has several properties:

- width and height (integer): The with and height of the room
- spawn (pair of integers): The location where the player will spawn if starting in the room
- places (json dict with pairs of integers as values): Other named spawn locations. These can be used by portals in other rooms to take the player to a specific location in the room.
- field (list of strings): Indicates what kind of tile each location should have. The kind of tile is denoted with a single character (can be any unicode character) that can be looked up in the mapping. The list should be as long as the height, and each string should be as long as the width (though it will be cut off or filled in with empty tiles if it is too long or short).
- mapping (json dict): A dictionary that denotes what each character in the field corresponds to. The key is a single character. The value is either a template or a list of templates.

A template is either a string, or a json object with at least the property "type" (a string) and possibly the properties "args" (a list of parameters) and "kwargs" (a json dict of parameters). The usage of "args" is discouraged.

The following templates are equivalent: `"grass"`, `{"type": "grass"}`, `{"type": "grass", "args": [], "kwargs": {}}`

A parameter can be a string, an integer, a float, a boolean, a list (not the same as a json list) or a template.
The specific type can be given explicitly by giving a list of 2 elements: the first is the type (as string), the second the actual value.
For example: `["int", 3]`.
For integers, floats and booleans the type can be derived from the json value.
Templates need to be in the object form or with explicit type annotation because they would otherwise be seen as strings.
Lists always need the type annotation.
For example a list of pairs of a template and a float would look like this: `["list", [["list", [{"type": "carrotseed"}, 1.0]],["list", [{"type": "carrot"}, 1.0]]]]`

The type of a template refers to an assemblage in the encyclopedia.
The args and kwargs of the template are arguments to that assemblage.
The encyclopedia has to be checked to see what arguments an assemblage uses and how it uses them.


# Example


	{
		"width": 43,
		"height": 23,
		"spawn": [6, 19],
		"field": [
			" ~~~~~XXXXXXXXXXXX~~~XXXXXXXXXXXXXXXXXXXXXX",
			" ~~~~~,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,,X",
			" ~~~~,,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,,X",
			" ~~~~,,,,,,,,,,r,,~~~~,,,,,,,,,,,,,,,,,,,,X",
			" ~bbbb..,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			" ~~~~,,.,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			" ~~~,,,.,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			" ~~,,,,.,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,,X",
			" X,,,,,.,,,,,,,,,,,~~~~,,,,,,T,,,,,,,,,,,,X",
			" X,,,,,.,,,,,,,,,,,,~~~,,,,,,,,,,,,,,,,,,,X",
			" X,^,,,.,,,,,,,,,,,,~~~,,,,,T,,,,######,,,X",
			" X,^,,,.,,,,,,,,,,,,bbb,,,,,,,,,,#++++#,,,X",
			" X,,,t..............bbb..........D++++#,,,X",
			" X,**,,.,,,,,,,,,,,,bbb,,,,,,,,,,#++++#,,,X",
			" X,*,*,.,u,,,V,,V,,,~~~,,,T,,,T,,#++++#,,,X",
			" X,,*,,.,,,,,,,,,,,,~~~,,,,,,,,,,######,,,X",
			" X,oo,,.,s,d,,,,,,,~~~~,,,,,,,,,,f,,,,f,,,X",
			" X,,*,,.,,,,,,,,,,,~~~''''''''''''''''f'''X",
			" X*,,,,.,,,d,VVV,,,~~~'''''''''''f''''f'''X",
			"1.......,,,,,VVV,,,~~~'''''''''''ffffff'''X",
			" X/,,,,.,P,Q,VVV,,,~~~''''''''''''''''''''X",
			" XXXXX,.,XXXXXXXXXX~~~XXXXXXXXXXXXXXXXXXXXX",
			"      %%%                                  "
		],
		"mapping": {
			"#": "wall",
			",": "grass",
			".": "ground",
			"~": "water",
			"b": "bridge",
			"+": "floor",
			"'": "greengrass",
			"T": ["grass", "tree"],
			"f": ["grass", "fence"],
			"X": "rock",
			"*": ["grass", {"type": "spawner", "kwargs": {"template": {"type": "pebble"}, "delay": 1200, "initial_spawn": false}}],
			"o": ["grass", {"type": "spawner", "kwargs": {"template": {"type": "stone"}, "delay": 1200, "initial_spawn": false}}],
			"%": {"type": "portal", "kwargs": {"destination": "broom", "destpos": "northentry"}},
			"1": {"type": "portal", "kwargs": {"destination": "smallview"}},
			"^": ["grass", "spiketrap"],
			"d": ["grass", {"type": "spawner", "kwargs": {"template": {"type": "dummy"}, "delay": 100, "initial_spawn": true}}],
			"r": ["grass", {"type": "spawner", "kwargs": {"template": {"type": "rat"}, "amount": 3, "clan": "rats", "delay": 200, "initial_spawn": true}}],
			"V": ["grass", "radishplant"],
			"/": ["grass", "sword"],
			"D": ["ground", "closeddoor"],
			"s": ["ground", "sign"],
			"u": ["ground", "dude"],
			"t": ["ground", "trader"],
			"P": ["ground", "pickaxe"],
			"Q": "quarry",
			" ": []
		}
	}
