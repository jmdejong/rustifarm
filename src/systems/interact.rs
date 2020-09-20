
use std::collections::HashSet;
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
		Minable
	},
	controls::{Control},
	resources::{Ground, Emigration},
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
		WriteStorage<'a, Minable>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut cooldowns, interactables, mut triggerbox, mut ears, inventories, visibles, players, mut emigration, mut whitelists, mut minables): Self::SystemData) {
		for (actor, controller, position) in (&entities, &controllers, &positions).join(){
			let mut target = None;
			let ear = ears.get_mut(actor);
			match &controller.control {
				Control::Interact(directions, arg) => {
					for (ent, interactable) in ground.components_near(position.pos, directions, &interactables) {
						if interactable.accepts_arg(arg){
							target = Some((ent, interactable, arg.clone()));
							break;
						}
					}
				}
				_ => {}
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
									if let Some(set) = whitelist.allowed.get(&destination.name) {
										if set.contains(&player.id){
											emigration.emigrants.push((player.id.clone(), destination, RoomPos::Unknown));
										} else {
											say(ear, format!("not allowed to visit {}", playername), name);
										}
									} else {
										say(ear, format!("unknown destination {}", playername), name);
									}
								} else if let Some(playername) = strip_prefix(&argument, "allow ") {
									let destination = dest.format(hashmap!("{player}" => player.id.name.as_str()));
									whitelist.allowed.entry(destination.name).or_insert_with(HashSet::new).insert(PlayerId{name: playername.to_string()});
									say(ear, format!("allowed {} to enter your home", playername), name);
								} else if let Some(playername) = strip_prefix(&argument, "disallow ") {
									let destination = dest.format(hashmap!("{player}" => player.id.name.as_str()));
									whitelist.allowed.entry(destination.name).or_insert_with(HashSet::new).remove(&PlayerId{name: playername.to_string()});
									say(ear, format!("disallowed {} to enter your home", playername), name);
								} else if argument.starts_with("whitelist") {
									let destination = dest.format(hashmap!("{player}" => player.id.name.as_str()));
									let allowed = whitelist.allowed.entry(destination.name).or_insert_with(HashSet::new).iter().map(|id| id.name.as_str()).collect::<Vec<&str>>();
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
						if let (Some(inventory), Some(minable)) = (inventories.get(actor), minables.get_mut(ent)) {
							let stats = inventory.equipment_bonuses();
							if let Some(skill_value) = stats.get(skill) {
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
