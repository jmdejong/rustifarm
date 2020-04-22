# Glossary


## Pre-entity

A pre-entity is a list of components with all their properties filled in.
This is used to construct the in-game entities.

## Assemblage

An assemblage is a kind of object type or blueprint.
Most properties of the entity are defined here, but some things can still be customized with arguments.
These arguments can be filled in by a template in order to construct a pre-entity.

## Encyclopedia

An encyclopedia names and holds many assemblages.
When you have a template you can look up the appropriate assemblage in the encyclopedia to construct a pre-entity.
Currently a game only has one encyclopedia instance.

## Template

A combination of an assemblage name (as defined in the encyclopedia) and arguments to that assemblage.
Using an encyclopedia this can be constructed into an entity.
Considering that the encyclopedia does not really change this is the smallest way to describe an entity.

## RoomTemplate

The definition of static objects in the room, as described in map_format.md

## Item

Something that exists in the player inventory. Usually this also corresponds to an assemblage that can be picked up and produces this item when picked up.

## Parameter

A layer between JSON and the datastructures in the rustifarm code. Can be int, float, bool, string, template, list and interaction.

## ComponentParameter

Either a Parameter, or a special function to that can evaluate a parameter, for example the value of an argument.
Its value is computed at the moment that a pre-entity is constructed from an assemblage and a template.
