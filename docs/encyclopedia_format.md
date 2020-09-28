
# Encyclopedia

In asciifarm all the assemblages and items are stored in an encyclopedia.

See https://github.com/jmdejong/rustifarm/blob/master/content/encyclopediae/default_encyclopedia.json for an example.

Encyclopediae definitions can use the [JSON5](https://json5.org/) format.

An encyclopedia file has a json object that can have the properties "assemblages", "items", "substitute", "item_substitute" and "assemblage_substitute".
"assemblages" and "items" are required.
"assemblages" is a dict of assemblages and "items" is a dict of items.
The keys can be the same, but by default the items dict will automatically insert an assemblage of the same name in the assemblages dict.

## Assemblage

An assemblage is a type/blueprint of game object.
The assemblage can be futher customized using the given arguments.

There are 3 main properties of an assemblage: "components", "arguments", "save" and "extract".
All other properties are just shortcuts.

### Components

"components" is the most important property. This is a list.
The items of the list are a list of 2 items: The component name (as a string) and a dict of ParameterExpressions.

Example:

	"wall": {
		"components": [
			["Visible", {
				"sprite": "wall",
				"height": 0.1,
				"name": "wall"
			}],
			["Flags", {
				"flags": ["Blocking"]
			}]
		]
	}

Note: technically the encyclopedia key (`"wall":`) part is not part of the assemblage definition.

The Flags and Visible components are so common that there is a shortcut for defining them.
If the assemblage has a "sprite" property and a "height" property then in preprocessing a Visible component will be added.
A "name" property is optional for the shortcut and will default to the sprite name (this might change to default to the key in the encyclopedia).
The Flags component will automatically be added if the "flags" poperty is on the assemblage.

The example above can be shortened to this:

	"wall": {
		"sprite": "wall",
		"height": 2,
		"flags": ["Blocking"]
	}

For a full list of components and what type of parameters they take, see: https://github.com/jmdejong/rustifarm/blob/master/src/componentwrapper.rs (lower part of the file)

### Arguments

An assemblage can be constructed with a template.
This template can provide the assemblage with arguments.
The arguments are a dictinary.
The key item is the name (as string).
The value is the default value for the argument, or null to indicate that there is no default and that the template should provide one.

An "args" ParameterExpression in the component definitions will have its value filled in with the value that is given to this argument, or otherwise the default value (and if that doesn't exist either it will error).

Example: 

	"portal": {
		"arguments": {"destination": null, "destpos", ""},
		"components": [
			["RoomExit", {"destination": {"$arg": "destination"}, "dest_pos": {"$arg": "destpos"}}]
		],
		"flags": ["Floor"]
	}

Arguments (and other ParameterExpressions) can not be used within shortcuts, so if you want to set the sprite by argument you'll have to add the Visible component manually.


### Save

If the "save" property is not present in an assemblage, when construction the entity a serialize component will be added automatically unless the template specifies that it should not be saved (automatically the case for all entities added from the roomtemplate).
If this is set to false the entity is never saved.
If this is set to true the entity is always saved, even when the template says it shouldn't save.

Everytime the map is loaded the objects from its roomtemplate will be added again, so if save is set to true for one of these objects, it will add more and more instances whenever it loads. It is best to enable this only together with the Dedup component.

### Extract

The default serialization for an entity is the template with which it was constructed.
For most objects this is enough, but this is not always the case.
Some objects have important properties that change during its existence.
Examples are the health of an construction, or the time when a plant should grow to the next stage.
In this case the value has to be extracted from the components in the game. That's what extract is for.

the value of the "extract" property is a json dict where the key is the name of one of the arguments, and the value is a json list of 2 json strings.
The first is the component from which the property should be extracted. The second is the name of that property on the component (the same name that was given to it).
Not all components can have properties extracted, and some components can only have some properties extracted.

Example:

	"builtwall": {
		"arguments": {"health", 100},
		"components": [
			["Health", {"health": {"$arg": "health"}, "maxhealth": 100}],
			["Loot", {"loot": [[{"$template": "stone"}, 1.0]]}]
		],
		"sprite": "builtwall",
		"height": 2,
		"extract": {"health": ["Health", "health"]},
		"flags": ["Blocking"]
	},

## Items

Items are things that can exist in the player inventory.
They can be dropped, and most items can be used.
Items do not have any own data, except its type.

The "items" property in the encyclopedia is a json dict.
The keys are the names of the items. By default an assemblage of the same name will also be created.
The values are again a json dict that can have the following properties:

- entity (template): do not automatically create an assemblage when the item is dropped, but use this template instead for the dropped item.
- sprite (string): the sprite of the assemblage that is created when the item is dropped. Defaults to the key of the entry
- name (string): name of the item. Defaults to the key of the entry.
- action: the action that will be executed when using the item. The value of this is a list of two items: the action type and the argument. There are the following action types:
  - eat (argument is an int): Remove the item from inventory, and add argument to the current health,
  - build (argument is a list of 3 elements: first a template, second and third a list of flags (as strings)): Remove the item from the inventory, and build the template from its first argument at the current location. This is only possible if all the flags from the second argument are on the current tile, and none of the flags in the third argument.
  - equip (argument is a list of 2 elements: a string and a dict of numbers): the item is marked as equipped. If another equipped item is equipped and has the same slot (first argument) as this item, the other item will be unequipped. The second argument holds the stat improvements.

# Used concepts

## Parameter
A parameter can be a string, an integer, a float, a boolean, a list, or a template.
Most types map directoy to the corresponding json type.
A template is a json object that has a `:template` field.
The entity type is the value of that field.
Other arguments are passed in the same object.


## ParameterExpression

A ParameterExpression is either a parameter, or a special function.
The special functions are given in the same way as the type annotations of a parameter, so as a pair of the type of function (as string) and its argument.
The only place where ParameterExpressions currently occur is as parameters to components.

Possible ParameterExpressions:

- `$arg` (argument: string): take the actual value from the assemblage argument that is named by the argument of this ParameterExpression.
- `$template` (argument: entity type; also takes all other properties as parameters to the template). A Constructor for a template. The difference between a `:template` object and a `$template` object is that `:template` objects can only use constants as their parameters, while `$template` objects can have expressions in their parameters.
- `$random` (argument: list of ParameterExpressions): pick a random value from its arguments. All argument items must have the same type.
- `$concat` (argument: list of ParameterExpressions): concatenate the string value of its arguments. All argument items must be of type string.
- `$if` (argument: list of 3 ParameterExpressions: condition, thenpart, elsepart) if the condition evaluates to true, take the value of thenpart, otherwise take the value of elsepart. The condition must be of type bool, and the thenpart and elsepart must have the same type.
- `$self` (does not use its argument): the template given to the assemblage used to construct this pre-entity.
- `$name` (does not use its argument): the name of the assemblage in the encyclopedia.
