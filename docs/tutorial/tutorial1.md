# Rustifarm content creation tutorial

This tutorial covers the basics of how to make maps and objects for rustifarm.
Before starting this tutorial you should be able to run the server either by running the binary or by using cargo.
See the [Readme](https://github.com/jmdejong/rustifarm/blob/master/README.md#installationrunning) for installation instructions.

The content usually consists of two parts:  
- The encyclopediae define what objects and items exist and what they do
- The maps define what items are placed in a room and where

Encyclopediae can be found in the `encyclopediae` directory within the content directory, and maps can be found in the `maps` directory.
Maps are sometimes also called rooms.

For the first part of the tutorial all the default encyclopedia from the game will be used.

## Starting with a different content directory

This tutorial uses its own content directory: tutorial/content1/
The encyclopediae directory within this content directory is symlinked to the encyclopediae directory from the default content directory for rustifarm.
If this link does not work you can try copying that directory (so copy rustifarm/content/encyclopediae/ to /rustifarm/tutorial/content0/encyclopediae/).

To run the server with this content directory use the `-c` flag.

If you have a binary called `asciifarm` run:

	./asciifarm -c path/to/tutorial/content1

Or if you are running it using cargo, run:

	cargo run -- -c path/to/tutorial/content1

## Simple map definition

Here is the definition for a little island room:

	{
		"width": 16,
		"height": 16,
		"spawn": [7, 8],
		"field": [
			"~~~~~~~~~~~~~~~~",
			"~~~~~~~~~~~~~~~~",
			"~~~~,,,,,,,,~~~~",
			"~~~,,,#####,,~~~",
			"~~,,,,#+++#,,,~~",
			"~~,,,,#+++#,,,~~",
			"~~,,,,##D##,,,~~",
			"~~,,,,,,.,,,,,~~",
			"~~,,T,,,.,,,,,~~",
			"~~,,,,,,.,,,,,~~",
			"~~,,,T,,.,,,,,~~",
			"~~,,,,,,.,,T,,~~",
			"~~~,,,,,.,,,,~~~",
			"~~~~,,,,.,,,~~~~",
			"~~~~~~~~~~~~~~~~",
			"~~~~~~~~~~~~~~~~"
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
			"D": ["ground", "closeddoor"],
			" ": []
		}
	}

The "width" and "height" fields define the size of the room (16 by 16 in this case).

The "spawn" field defines where new players spawn in this room. In this case that is on location x: 7, y: 8.

The "field" field is a list of strings that represent all the tiles.
Each character corresponds to a certain object or a list of objects.

What object a character corresponds to is defined in the "mapping" field.
<!-- Each character in "field" must have a matching enty in "mapping" (otherwise the room can not be loaded). -->

In this example the '~' character corresponds to a tile with one object: 'water'.
The water object is defined in the encyclopedia.

Similarily, the ',' corresponds to a tile with grass.

Sometimes it is necessary to have more that one object on the same tile, like for the door. In this case the entry can be a list of tiles.
In fact, `"X": "rock"` is just a shortcut for `"X": ["rock"]`.

## Tile definitions

Let's take a look at some definitions for simple tiles.

	"water": {
		"sprite": "water",
		"height": 0.0
	}

Water has a very simple definition: it is using the sprite named "water" and it has a height of 0.
The height is used to determine the drawing order when multiple objects are in the same tile: an object with a larger height will always be drawn on top of an object with a smaller height.
In this case it does not really matter since there are no other objects on the same tile.

	"ground": {
		"sprite": "ground",
		"height": 0.1,
		"flags": ["Floor", "Soil"]
	},

The ground is simple too.
It has the "ground" tile, and it is a bit taller than water, but still not very tall.
The gound also has some flags: "Floor" and "Soil".
"Floor" means that the player can walk on that tile.
Players (and other objects as well) can only walk on tiles that have at least one object with a "Floor" flag.
The "Soil" flag means that it is possible to plant plants here

	"wall": {
		"sprite": "wall",
		"height": 2,
		"flags": ["Blocking"]
	},

Walls are a bit taller.
Walls have the blocking flag which means that it is not possible to walk to that square, even when there is a "Floor" flag.

To clarify the walking: you can walk to a tile if and only if there that tile has at least one object with a "Floor" flag and no objects with the "Blocking" flag.

