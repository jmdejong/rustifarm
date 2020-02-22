

use specs::{
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use crate::components::{
	Controller,
	Position,
	Inventory,
	Health
};

use crate::resources::{NewEntities};
use crate::components::item::ItemAction::{None, Build, Eat};
use crate::controls::Control;


pub struct Use;
impl <'a> System<'a> for Use {
	type SystemData = (
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Inventory>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Health>
	);
	
	fn run(&mut self, (controllers, positions, mut inventories, mut new, mut healths): Self::SystemData) {
		for (controller, position, inventory, maybe_health) in (&controllers, &positions, &mut inventories, (&mut healths).maybe()).join(){
			match &controller.0 {
				Control::Use(rank) => {
					if let Some(item) = inventory.items.get(*rank) {
						match &item.action {
							Build(template) => {
								let _ = new.create(position.pos, template.clone());
								inventory.items.remove(*rank);
							}
							Eat(health_diff) => {
								if let Some(health) = maybe_health {
									health.heal(*health_diff);
								}
								inventory.items.remove(*rank);
							}
							None => {}
						}
					}
				}
				_ => {}
			}
		}
	}
}
