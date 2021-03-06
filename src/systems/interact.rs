
use std::collections::{HashSet, HashMap};
use rand::Rng;

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
		Whitelist,
		Minable,
		Stats
	},
	controls::{Control},
	resources::{Ground, Emigration, NewEntities},
	hashmap,
	playerstate::RoomPos,
	PlayerId,
	util::strip_prefix
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
		WriteStorage<'a, Ear>,
		WriteStorage<'a, Inventory>,
		ReadStorage<'a, Visible>,
		ReadStorage<'a, Player>,
		Write<'a, Emigration>,
		WriteStorage<'a, Whitelist>,
		WriteStorage<'a, Minable>,
		ReadStorage<'a, Stats>,
		Read<'a, NewEntities>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut cooldowns, interactables, mut triggerbox, mut ears, mut inventories, visibles, players, mut emigration, mut whitelists, mut minables, stats, new): Self::SystemData) {
		for (actor, controller, position) in (&entities, &controllers, &positions).join(){
			let mut target = None;
			let ear = ears.get_mut(actor);
			if let Control::Interact(directions, arg) = &controller.control {
				for (ent, interactable) in ground.components_near(position.pos, directions, &interactables) {
					if interactable.accepts_arg(arg){
						target = Some((ent, interactable, arg.clone()));
						break;
					}
				}
			}
			if let Some((ent, interactable, arg)) = target {
				let mut cooldown = 2;
				let name = visibles.get(ent).map(|v| v.name.as_str());
				match interactable {
					Interactable::Say(text) => {
						say(ear, text.clone(), name);
					}
					Interactable::Reply(text) => {
						say(ear, text.replace("{}", &arg.unwrap()), name);
					}
					Interactable::Trigger(trigger) => {
						TriggerBox::add_message(&mut triggerbox, ent, *trigger);
					}
					Interactable::Visit(dest) => {
						if let Some(argument) = arg {
							if let (Some(player), Some(whitelist)) = (players.get(actor), whitelists.get_mut(ent)){
								if let Some(playername) = strip_prefix(&argument, "visit ") {
									let destination = dest.format(hashmap!("{player}" => playername));
									if let Some(set) = whitelist.allowed.get(&destination.0) {
										if set.contains(&player.id){
											emigration.emigrants.push((player.id.clone(), destination, RoomPos::Unknown));
										} else {
											say(ear, format!("not allowed to visit {}", playername), name);
										}
									} else {
										say(ear, format!("unknown destination {}", playername), name);
									}
								} else if let Some(playername) = strip_prefix(&argument, "allow ") {
									let destination = dest.format(hashmap!("{player}" => player.id.0.as_str()));
									whitelist.allowed.entry(destination.0).or_insert_with(HashSet::new).insert(PlayerId( playername.to_string()));
									say(ear, format!("allowed {} to enter your home", playername), name);
								} else if let Some(playername) = strip_prefix(&argument, "disallow ") {
									let destination = dest.format(hashmap!("{player}" => player.id.0.as_str()));
									whitelist.allowed.entry(destination.0).or_insert_with(HashSet::new).remove(&PlayerId( playername.to_string()));
									say(ear, format!("disallowed {} to enter your home", playername), name);
								} else if argument.starts_with("whitelist") {
									let destination = dest.format(hashmap!("{player}" => player.id.0.as_str()));
									let allowed = whitelist.allowed.entry(destination.0).or_insert_with(HashSet::new).iter().map(|id| id.0.as_str()).collect::<Vec<&str>>();
									say(ear, format!("allowed players: {}", allowed.join(", ")), name);
								}
							}
						} else if let Some(ear) = ear {
							ear.sounds.push(Notification::Options{
								description: "Portal".to_string(),
								options: vec![
									("visit <player>", "visit <player>. Only possible if they have allowed you"),
									("allow <player>", "allow <player> to visit you"),
									("disallow <player>", "disallow <player> to visit you"),
									("whitelist", "show the list of players allowed to visit you")
								].iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
							})
						}
					}
					Interactable::Mine(skill) => {
						if let Some(minable) = minables.get_mut(ent) {
							let mut skills = inventories.get(actor).map(Inventory::equipment_bonuses).unwrap_or_else(HashMap::new);
							if let Some(skillset) = stats.get(actor) {
								for (skill, val) in skillset.skills.iter() {
									*skills.entry(*skill).or_insert(0) += val;
								}
							}
							if let Some(skill_value) = skills.get(skill) {
								println!("{:?} {:?}", skill, skill_value);
								// todo: give player feedback
								cooldown = 20;
								minable.progress += rand::thread_rng().gen_range(0, skill_value+1);
								if minable.progress >= minable.total {
									TriggerBox::add_message(&mut triggerbox, ent, minable.trigger);
									minable.progress = 0;
								}
							}
						}
					}
					Interactable::Exchange(prefix, exchanges) => {
						if let Some(txt) = arg {
							if let (Some(inventory), Some(action)) = (inventories.get_mut(actor), strip_prefix(&txt, prefix)) {
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
						} else if let Some(ear) = ear {
							ear.sounds.push(Notification::Options{
								description: "".to_string(),
								options: exchanges.iter().map(|(id, exchange)| (format!("{}{}", prefix, id), exchange.show())).collect()
							})
						}
					}
				}
				cooldowns.insert(actor, ControlCooldown{amount: cooldown}).unwrap();
			}
		}
	}
}

fn say(maybe_ear: Option<&mut Ear>, text: String, source: Option<&str>){
	if let Some(ear) = maybe_ear {
		ear.sounds.push(Notification::Sound{text, source: source.map(|s| s.to_string())});
	}
}
