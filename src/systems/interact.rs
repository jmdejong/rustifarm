
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
		TriggerBox,
		Notification,
		Ear,
		Inventory,
		Visible,
		Player,
		Whitelist
	},
	controls::{Control},
	resources::{Ground, NewEntities, Emigration},
	hashmap,
	playerstate::RoomPos,
	PlayerId
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
		WriteStorage<'a, TriggerBox>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Ear>,
		WriteStorage<'a, Inventory>,
		ReadStorage<'a, Visible>,
		ReadStorage<'a, Player>,
		Write<'a, Emigration>,
		WriteStorage<'a, Whitelist>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut cooldowns, interactables, mut triggerbox, new, mut ears, mut inventories, visibles, players, mut emigration, mut whitelists): Self::SystemData) {
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
				let name = visibles.get(ent).map(|v| v.name.as_str());
				match interactable {
					Interactable::Trigger(trigger) => {
						TriggerBox::add_message(&mut triggerbox, ent, *trigger);
					}
					Interactable::Say(text) => {
						say(ear, text.clone(), name);
					}
					Interactable::Reply(text) => {
						say(ear, text.replace("{}", &arg.unwrap()), name);
					}
					Interactable::Exchange(prefix, exchanges) => {
						if let Some(txt) = arg {
							if let Some(inventory) = inventories.get_mut(entity) {
								if txt.starts_with(prefix){
									let action = txt.split_at(prefix.len()).1;
									if let Some(exchange) = exchanges.get(action) {
										if exchange.can_trade(inventory){
											exchange.trade(inventory, &new.encyclopedia);
											say(ear, format!("Success! '{}' ({})", txt, exchange.show()), name);
										} else {
											say(ear, format!("You do not have the required items or inventory space for '{}' ({})", txt, exchange.show()), name);
										}
									} else {
										say(ear, format!("Invalid option: {}", action), name);
									}
								}
							}
						} else {
							say(
								ear,
								format!("options: {:?}", exchanges.iter().map(|(id, exchange)| 
									format!("{}{}: {}", prefix, id, exchange.show())
								).collect::<Vec<String>>()),
								name
							);
						}
					}
					Interactable::Visit(dest) => {
						if let (Some(player), Some(whitelist)) = (players.get(entity), whitelists.get_mut(ent)){
							let argument = arg.unwrap();
							if argument.starts_with("visit ") {
								let playername = argument.split_at("visit ".len()).1;
								let destination = dest.format(hashmap!("{player}" => playername));
								if let Some(set) = whitelist.allowed.get(&destination.name) {
									if set.contains(&player.id){
										emigration.emigrants.push((player.id.clone(), destination, RoomPos::Unknown));
									} else {
										say(ear, format!("not allowed to visit {}", playername), name);
									}
								} else {
									say(ear, format!("unknown destination {}", playername), name);
								}
							} else if argument.starts_with("allow ") {
								let playername = argument.split_at("allow ".len()).1;
								let destination = dest.format(hashmap!("{player}" => player.id.name.as_str()));
								whitelist.allowed.entry(destination.name).or_insert_with(HashSet::new).insert(PlayerId{name: playername.to_string()});
								say(ear, format!("allowed {} to enter your home", playername), name);
							} else if argument.starts_with("disallow ") {
								let playername = argument.split_at("disallow ".len()).1;
								let destination = dest.format(hashmap!("{player}" => player.id.name.as_str()));
								whitelist.allowed.entry(destination.name).or_insert_with(HashSet::new).remove(&PlayerId{name: playername.to_string()});
								say(ear, format!("disallowed {} to enter your home", playername), name);
							} else if argument.starts_with("whitelist") {
								let destination = dest.format(hashmap!("{player}" => player.id.name.as_str()));
								let allowed = whitelist.allowed.entry(destination.name).or_insert_with(HashSet::new).iter().map(|id| id.name.as_str()).collect::<Vec<&str>>();
								say(ear, format!("allowed players: {}", allowed.join(", ")), name);
							}
						}
					}
				}
				cooldowns.insert(entity, ControlCooldown{amount: 2}).unwrap();
			}
		}
	}
}

fn say(maybe_ear: Option<&mut Ear>, text: String, source: Option<&str>){
	if let Some(ear) = maybe_ear {
		ear.sounds.push(Notification::Sound{text, source: source.map(|s| s.to_string())});
	}
}
