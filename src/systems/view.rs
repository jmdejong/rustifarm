
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

use crate::{Pos, Sprite};
use crate::components::{Visible, Player, Position, Inventory, New, Moved, Removed, Health};
use crate::resources::{Size, Output, Ground};
use crate::worldmessages::{WorldMessage, FieldMessage};


#[derive(Default)]
pub struct View;

impl <'a> System<'a> for View {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Inventory>,
		ReadStorage<'a, Health>,
		ReadStorage<'a, Visible>,
		Read<'a, Size>,
		ReadStorage<'a, Player>,
		Write<'a, Output>,
		ReadStorage<'a, New>,
		ReadStorage<'a, Moved>,
		ReadStorage<'a, Removed>,
		Read<'a, Ground>
	);
	fn run(&mut self, (entities, positions, inventories, healths, visible, size, players, mut output, new, moved, removed, ground): Self::SystemData) {
		
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
		let mut changes: Vec<(Pos, Vec<Sprite>)> = Vec::new();
		for pos in changed {
			changes.push((pos, cell_sprites(ground.cells.get(&pos).unwrap_or(&HashSet::new()), &visible)));
		}
		output.output.clear();
		
		for (ent, player, pos) in (&entities, &players, &positions).join() {
			let mut updates = WorldMessage::default();
			if new.get(ent).is_some() {
				let (values, mapping) = draw_room(&ground.cells, (size.width, size.height), &visible);
				let field = FieldMessage{
					width: size.width,
					height: size.height,
					field: values,
					mapping
				};
				updates.field = Some(field);
			} else if has_changed {
				updates.change = Some(changes.clone());
			}
			if let Some(inventory) = inventories.get(ent){
				updates.inventory = Some(inventory.items.iter().map(|item| item.name.clone()).collect());
			}
			if let Some(health) = healths.get(ent){
				updates.health = Some((health.health, health.maxhealth));
			}
			updates.pos = Some(pos.pos);
			if !updates.is_empty() {
				output.output.insert(player.id.clone(), updates);
			}
		}
	}
}

fn cell_sprites(entities: &HashSet<Entity>, visible: &ReadStorage<Visible>) -> Vec<Sprite> {
	let mut visibles: Vec<&Visible> = entities.iter().filter_map(|ent| visible.get(*ent)).collect();
	visibles.sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
	visibles.iter().map(|vis| vis.sprite.clone()).collect()
}

fn draw_room(ground: &HashMap<Pos, HashSet<Entity>>, (width, height): (i64, i64), visible: &ReadStorage<Visible>) -> (Vec<usize>, Vec<Vec<Sprite>>){
	
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<Sprite> = match ground.get(&Pos{x, y}) {
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
