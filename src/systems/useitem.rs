

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write,
	Read
};

use crate::{
	components::{
		Controller,
		Position,
		Inventory,
		AttackInbox,
		AttackMessage,
		AttackType,
		Flags
	},
	resources::{NewEntities, Ground},
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
		Read<'a, Ground>,
		ReadStorage<'a, Flags>
	);
	
	fn run(&mut self, (entities, controllers, positions, mut inventories, mut new, mut attacked, ground, flags): Self::SystemData) {
		for (ent, controller, position, inventory) in (&entities, &controllers, &positions, &mut inventories).join(){
			match &controller.control {
				Control::Use(rank) => {
					if let Some(entry) = inventory.items.get_mut(*rank) {
						match &entry.0.action {
							Build(template, required_flags, blocking_flags) => {
								let ground_flags = ground.flags_on(position.pos, &flags);
								if required_flags.is_subset(&ground_flags) && blocking_flags.is_disjoint(&ground_flags){
									new.create(position.pos, &template).unwrap();
									inventory.items.remove(*rank);
								}
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
