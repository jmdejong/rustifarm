
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
		Exchanger,
		Notification,
		Ear,
		Inventory,
		Visible
	},
	controls::{Control},
	resources::{Ground, NewEntities},
	util::strip_prefix
};

pub struct Exchange;
impl <'a> System<'a> for Exchange {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		ReadStorage<'a, Position>,
		Read<'a, Ground>,
		ReadStorage<'a, Exchanger>,
		Write<'a, NewEntities>,
		WriteStorage<'a, Ear>,
		WriteStorage<'a, Inventory>,
		ReadStorage<'a, Visible>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, exchangers, new, mut ears, mut inventories, visibles): Self::SystemData) {
		for (actor, controller, position) in (&entities, &controllers, &positions).join(){
			let ear = ears.get_mut(actor);
			match &controller.control {
				Control::Interact(directions, arg) => {
					for (ent, exchanger) in ground.components_near(position.pos, directions, &exchangers) {
						let prefix = exchanger.prefix.as_str();
						let name = visibles.get(ent).map(|v| v.name.as_str());
						if let Some(txt) = arg {
							if let (Some(inventory), Some(action)) = (inventories.get_mut(actor), strip_prefix(&txt, prefix)) {
								if let Some(exchange) = exchanger.exchanges.get(action) {
									if exchange.can_trade(inventory){
										exchange.trade(inventory, &new.encyclopedia);
										say(ear, format!("Success! '{}' ({})", txt, exchange.show()), name);
									} else {
										say(ear, format!("You do not have the required items or inventory space for '{}' ({})", txt, exchange.show()), name);
									}
								} else {
									say(ear, format!("Invalid option: {}", action), name);
								}
							break;
							}
						} else if let Some(ear) = ear {
							ear.sounds.push(Notification::Options{
								description: "".to_string(),
								options: exchanger.exchanges.iter().map(|(id, exchange)| (format!("{}{}", prefix, id), exchange.show())).collect()
							});
							break;
						}
					}
				}
				_ => {}
			}
		}
	}
}

fn say(maybe_ear: Option<&mut Ear>, text: String, source: Option<&str>){
	if let Some(ear) = maybe_ear {
		ear.sounds.push(Notification::Sound{text, source: source.map(|s| s.to_string())});
	}
}
