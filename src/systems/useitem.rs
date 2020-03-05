

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
		AttackType,
		Equipment
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
		WriteStorage<'a, AttackInbox>,
		WriteStorage<'a, Equipment>
	);
	
	fn run(&mut self, (entities, controllers, positions, mut inventories, mut new, mut attacked, mut equipments): Self::SystemData) {
		for (ent, controller, position, inventory) in (&entities, &controllers, &positions, &mut inventories).join(){
			match &controller.control {
				Control::Use(rank) => {
					if let Some(item) = inventory.items.get(*rank) {
						match &item.action {
							Build(template) => {
								new.create(position.pos, template.clone()).unwrap();
								inventory.items.remove(*rank);
							}
							Eat(health_diff) => {
								AttackInbox::add_message(&mut attacked, ent, AttackMessage{typ: AttackType::Heal(*health_diff), attacker: Option::None});
								inventory.items.remove(*rank);
							}
							Equip(equippable) => {
								if let Some(equipment) = equipments.get_mut(ent) {
									if equipment.equipment.contains_key(&equippable.slot) {
										equipment.equipment.insert(equippable.slot, Some(equippable.clone()));
									}
								}
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
