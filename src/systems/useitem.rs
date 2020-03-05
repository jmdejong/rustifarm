

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use crate::{
	components::{
		Controller,
		Position,
		Inventory,
		AttackInbox,
		AttackMessage,
		AttackType
	},
	resources::{NewEntities},
	components::item::ItemAction::{None, Build, Eat, Equip},
	controls::Control,
};


pub struct Use;
impl <'a> System<'a> for Use {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		WriteStorage<'a, Inventory>,
		Write<'a, NewEntities>,
		WriteStorage<'a, AttackInbox>
	);
	
	fn run(&mut self, (entities, controllers, positions, mut inventories, mut new, mut attacked): Self::SystemData) {
		for (ent, controller, position, inventory) in (&entities, &controllers, &positions, &mut inventories).join(){
			match &controller.control {
				Control::Use(rank) => {
					if let Some(entry) = inventory.items.get_mut(*rank) {
						match &entry.0.action {
							Build(template) => {
								new.create(position.pos, template.clone()).unwrap();
								inventory.items.remove(*rank);
							}
							Eat(health_diff) => {
								AttackInbox::add_message(&mut attacked, ent, AttackMessage{typ: AttackType::Heal(*health_diff), attacker: Option::None});
								inventory.items.remove(*rank);
							}
							Equip(equippable) => {
								let slot = equippable.slot;
								for otherentry in inventory.items.iter_mut() {
									if let Equip(other) = &otherentry.0.action {
										if other.slot == slot {
											otherentry.1 = false;
										}
									}
								}
								inventory.items[*rank].1 = true;
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
