
use std::collections::HashSet;
use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Read,
	Write
};

use crate::{
	components::{
		Controller,
		Position,
		ControlCooldown,
		Interactable,
		Dead,
		Sound,
		Ear,
		Inventory
	},
	controls::{Control},
	resources::{Ground, NewEntities}
};

pub struct Interact;
impl <'a> System<'a> for Interact {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>,
		WriteStorage<'a, ControlCooldown>,
		ReadStorage<'a, Interactable>,
		WriteStorage<'a, Dead>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Ear>,
		WriteStorage<'a, Inventory>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut cooldowns, interactables, mut deads, new, mut ears, mut inventories): Self::SystemData) {
		for (entity, controller, position) in (&entities, &controllers, &positions).join(){
			let mut target = None;
			let ear = ears.get_mut(entity);
			match &controller.control {
				Control::Interact(directions, arg) => {
					'targets: for direction in directions {
						let pos = position.pos + direction.to_position();
						for ent in ground.cells.get(&pos).unwrap_or(&HashSet::new()) {
							if let Some(interactable) = interactables.get(*ent) {
								if interactable.accepts_arg(arg){
									target = Some((*ent, interactable, arg.clone()));
									break 'targets;
								}
							}
						}
					}
				}
				_ => {}
			}
			if let Some((ent, interactable, arg)) = target {
				match interactable {
					Interactable::Harvest => {
						deads.insert(ent, Dead).unwrap();
					}
					Interactable::Say(text) => {
						say(ear, text.clone());
					}
					Interactable::Reply(text) => {
						say(ear, text.replace("{}", &arg.unwrap()));
					}
					Interactable::Exchange(prefix, exchanges) => {
						if let Some(txt) = arg {
							if let Some(inventory) = inventories.get_mut(entity) {
								if txt.starts_with(prefix){
									let action = txt.split_at(prefix.len()).1;
									if let Some(exchange) = exchanges.get(action) {
										if exchange.can_trade(inventory){
											exchange.trade(inventory, &new.encyclopedia);
											say(ear, format!("Success! '{}' ({})", txt, exchange.show()));
										} else {
											say(ear, format!("You do not have the required items or inventory space for '{}' ({})", txt, exchange.show()));
										}
									} else {
										say(ear, format!("Invalid option: {}", action));
									}
								}
							}
						} else {
							say(ear, format!("options: {:?}", exchanges.iter().map(|(id, exchange)| 
									format!("{}{}: {}", prefix, id, exchange.show())
								).collect::<Vec<String>>())
							);
						}
					}
				}
				cooldowns.insert(entity, ControlCooldown{amount: 2}).unwrap();
			}
		}
	}
}

fn say(maybe_ear: Option<&mut Ear>, text: String){
	if let Some(ear) = maybe_ear {
		ear.sounds.push(Sound{source: None, text});
	}
}
