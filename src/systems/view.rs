
use std::collections::{HashMap, HashSet};

use specs::{
	ReadStorage,
	Read,
	Write,
	System,
	Join,
	Entities,
	Entity
};

use super::super::pos::Pos;
use super::super::components::{Visible, Player, Position, Inventory, New, Moved, Removed};
use super::super::resources::{Size, Output, Ground};
use super::super::worldmessages::{WorldMessage, WorldUpdate, FieldMessage};


#[derive(Default)]
pub struct View;

impl <'a> System<'a> for View {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Inventory>,
		ReadStorage<'a, Visible>,
		Read<'a, Size>,
		ReadStorage<'a, Player>,
		Write<'a, Output>,
		ReadStorage<'a, New>,
		ReadStorage<'a, Moved>,
		ReadStorage<'a, Removed>,
		Read<'a, Ground>
	);
	fn run(&mut self, (entities, positions, inventories, visible, size, players, mut output, new, moved, removed, ground): Self::SystemData) {
		
		let mut changed = HashSet::new();
		for (pos, _new) in (&positions, &new).join() {
			changed.insert(pos.pos);
		}
		for (pos, mov) in (&positions, &moved).join() {
			changed.insert(pos.pos);
			changed.insert(mov.from);
		}
		for (pos, _removed) in (&positions, &removed).join() {
			changed.insert(pos.pos);
		}
		
		
		let has_changed: bool = !changed.is_empty();
		let mut changes: Vec<(Pos, Vec<String>)> = Vec::new();
		for pos in changed {
			changes.push((pos, cell_sprites(ground.cells.get(&pos).unwrap_or(&HashSet::new()), &visible)));
		}
		let changed_msg = WorldUpdate::Change(changes);
		
		
		output.output.clear();
		
		for (ent, player, pos) in (&entities, &players, &positions).join() {
			let mut updates: Vec<WorldUpdate> = Vec::new();
			if new.get(ent).is_some() {
				let (values, mapping) = draw_room(&ground.cells, (size.width, size.height), &visible);
				let field = WorldUpdate::Field(FieldMessage{
					width: size.width,
					height: size.height,
					field: values,
					mapping
				});
				updates.push(field);
			} else if has_changed {
				updates.push(changed_msg.clone());
			}
			if let Some(inventory) = inventories.get(ent){
				updates.push(WorldUpdate::Inventory(inventory.items.iter().map(|item| item.name.clone()).collect()));
			}
			updates.push(WorldUpdate::Pos(pos.pos));
			let message = WorldMessage{updates};
			output.output.insert(player.name.clone(), message);
		}
	}
}

fn cell_sprites(entities: &HashSet<Entity>, visible: &ReadStorage<Visible>) -> Vec<String> {
	let mut visibles: Vec<&Visible> = entities.iter().filter_map(|ent| visible.get(*ent)).collect();
	visibles.sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
	visibles.iter().map(|vis| vis.sprite.clone()).collect()
}

fn draw_room(ground: &HashMap<Pos, HashSet<Entity>>, (width, height): (i64, i64), visible: &ReadStorage<Visible>) -> (Vec<usize>, Vec<Vec<String>>){
	
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<String>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<String> = match ground.get(&Pos{x, y}) {
				Some(ents) => {cell_sprites(ents, visible)}
				None => {vec![]}
			};
			values.push(
				match mapping.iter().position(|x| x == &sprites) {
					Some(index) => {
						index
					}
					None => {
						mapping.push(sprites);
						mapping.len() - 1
					}
				}
			)
		}
	}
	(values, mapping)
}
