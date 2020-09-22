
use specs::{
	ReadStorage,
	WriteStorage,
	Read,
	Write,
	System,
	Join,
	Entities,
	Entity
};

use crate::{
	Pos,
	Sprite,
	components::{Visible, Player, Position, Inventory, New, Health, Ear},
	resources::{Size, Output, Ground},
	worldmessages::{WorldMessage, FieldMessage}
};

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
		Read<'a, Ground>,
		WriteStorage<'a, Ear>
	);
	fn run(&mut self, (entities, positions, inventories, healths, visible, size, players, mut output, new, ground, mut ears): Self::SystemData) {
		
		let changes: Vec<(Pos, Vec<Sprite>)> = ground.changes
			.iter()
			.map(|pos| (*pos, sprites_on(&ground, *pos, &visible, &inventories)))
			.collect();
		
		let has_changed: bool = !changes.is_empty();
		output.output.clear();
		
		for (ent, player, pos) in (&entities, &players, &positions).join() {
			let mut updates = WorldMessage::default();
			if new.get(ent).is_some() {
				let (values, mapping) = draw_room(&ground, (size.width, size.height), &visible, &inventories);
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
				updates.inventory = Some(inventory.items.iter().map(|entry| (entry.item.name.clone(), entry.is_equipped)).collect());
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
					.by_height(&pos.pos, &visible)
					.into_iter()
					.filter(|e| *e != ent)
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

fn entity_sprite(ent: Entity, visibles: &ReadStorage<Visible>, inventories: &ReadStorage<Inventory>) -> Option<Sprite> {
	if let Some(inventory) = inventories.get(ent) {
		if let Some(sprite) = inventory.equipment_sprites().into_iter().next() {
			return Some(sprite);
		}
	}
	if let Some(visible) = visibles.get(ent) {
		return Some(visible.sprite.clone());
	}
	None
}

fn sprites_on(ground: &Read<Ground>, pos: Pos, visibles: &ReadStorage<Visible>, inventories: &ReadStorage<Inventory>) -> Vec<Sprite> {
	ground.by_height(&pos, visibles)
		.into_iter()
		.filter_map(|e|
			entity_sprite(e, visibles, inventories)
		).collect()
}

fn draw_room(ground: &Read<Ground>, (width, height): (i64, i64), visible: &ReadStorage<Visible>, inventories: &ReadStorage<Inventory>) -> (Vec<usize>, Vec<Vec<Sprite>>){
	
	let size = width * height;
	let mut values :Vec<usize> = Vec::with_capacity(size as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	for y in 0..height {
		for x in 0..width {
			let sprites: Vec<Sprite> = sprites_on(ground, Pos{x, y}, visible, inventories);//cell_sprites(ground.components_on(Pos{x, y}, visible));
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
