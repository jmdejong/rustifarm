
use std::collections::{HashSet};

use specs::{
	ReadStorage,
	WriteStorage,
	Read,
	Write,
	System,
	Join,
	Entities
};

use crate::{
	Pos,
	Sprite,
	components::{Visible, Player, Position, Inventory, New, Moved, Removed, Health, Ear},
	resources::{Size, Output, Ground},
	worldmessages::{WorldMessage, FieldMessage}
};

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
		Read<'a, Ground>,
		WriteStorage<'a, Ear>
	);
	fn run(&mut self, (entities, positions, inventories, healths, visible, size, players, mut output, new, moved, removed, ground, mut ears): Self::SystemData) {
		
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
			changes.push((pos, cell_sprites(ground.components_on(pos, &visible, &removed))));
		}
		output.output.clear();
		
		for (ent, player, pos) in (&entities, &players, &positions).join() {
			let mut updates = WorldMessage::default();
			if new.get(ent).is_some() {
				let (values, mapping) = draw_room(&ground, (size.width, size.height), &visible, &removed);
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
				updates.inventory = Some(inventory.items.iter().map(|(item, _equipped)| item.name.clone()).collect());
			}
			if let Some(health) = healths.get(ent){
				updates.health = Some((health.health, health.maxhealth));
			}
			if let Some(ear) = ears.get_mut(ent){
				if !ear.sounds.is_empty(){
					updates.sounds = Some(ear.sounds.drain(..).map(|s| s.as_message()).collect());
				}
			}
			updates.ground = Some(
				ground
					.by_height(&pos.pos, &visible, &ent)
					.into_iter()
					.map(|ent| visible.get(ent).unwrap().name.clone())
					.collect()
			);
			updates.pos = Some(pos.pos);
			if !updates.is_empty() {
				output.output.insert(player.id.clone(), updates);
			}
		}
	}
}

fn cell_sprites(mut visibles: Vec<&Visible>) -> Vec<Sprite> {
	visibles.sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
	visibles.iter().map(|vis| vis.sprite.clone()).collect()
}

fn draw_room(ground: &Read<Ground>, (width, height): (i64, i64), visible: &ReadStorage<Visible>, removals: &ReadStorage<Removed>) -> (Vec<usize>, Vec<Vec<Sprite>>){
	
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<Sprite> = cell_sprites(ground.components_on(Pos{x, y}, visible, removals));
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
