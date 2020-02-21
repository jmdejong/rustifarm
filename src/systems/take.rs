
use std::collections::HashSet;

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use crate::components::{
	Controller,
	Position,
	Removed,
	Inventory,
	Item
};

use crate::controls::{Control};
use crate::resources::{Ground, NewEntities};



pub struct Take;
impl <'a> System<'a> for Take {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		Write<'a, Ground>,
		WriteStorage<'a, Removed>,
		ReadStorage<'a, Item>,
		WriteStorage<'a, Inventory>,
		Write<'a, NewEntities>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut removed, items, mut inventories, mut new): Self::SystemData) {
		for (ent, controller, position, inventory) in (&entities, &controllers, &positions, &mut inventories).join(){
			match &controller.0 {
				Control::Take(_rank) if inventory.items.len() < inventory.capacity => {
					let mut ents = ground.cells.get(&position.pos).unwrap_or(&HashSet::new()).clone();
					ents.remove(&ent);
					for ent in ents {
						if let Some(item) = items.get(ent) {
							inventory.items.insert(0, item.clone());
							if let Err(msg) = removed.insert(ent, Removed) {
								println!("{:?}", msg);
							}
							break;
						}
					}
				}
				Control::Drop(rank) => {
					if *rank >= inventory.items.len() {
						return
					}
					let item = inventory.items.remove(*rank);
					let _ = new.create(position.pos, item.ent);
				}
				_ => {}
			}
		}
	}
}
