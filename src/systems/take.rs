
use std::collections::HashSet;

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use super::super::pos::Pos;

use super::super::components::{
	Controller,
	Position,
	Removed,
	Inventory,
	Item
};

use super::super::controls::{Control};
use super::super::resources::{Ground, NewEntities};



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
				Control::Take(_rank) => {
					let mut ents = ground.cells.get(&position.pos).unwrap_or(&HashSet::new()).clone();
					ents.remove(&ent);
					for ent in ents {
						if let Some(item) = items.get(ent) {
							inventory.items.push(item.clone());
							if let Err(msg) = removed.insert(ent, Removed) {
								println!("{:?}", msg);
							}
						}
					}
				}
				Control::Drop(_rank) => {
					if let Some(item) = inventory.items.pop() {
						new.templates.push((position.pos, item.ent));
					}
				}
				_ => {}
			}
		}
	}
}
