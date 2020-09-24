
use crate::hashmap;
use specs::{
	ReadStorage,
	Read,
	Write,
	System,
	Join
};

use crate::components::{Player, Position, Moved, RoomExit};
use crate::resources::{Emigration, Ground};


pub struct Migrate;
impl <'a> System<'a> for Migrate {
	type SystemData = (
		Write<'a, Emigration>,
		Read<'a, Ground>,
		ReadStorage<'a, Player>,
		ReadStorage<'a, Position>,
		ReadStorage<'a, Moved>,
		ReadStorage<'a, RoomExit>,
		
	);
	fn run(&mut self, (mut emigration, ground, players, positions, moved, exits): Self::SystemData) {
	
		for (player, position, _moved) in (&players, &positions, &moved).join() {
			for ent in ground.cells.get(&position.pos).unwrap() {
				if let Some(exit) = exits.get(*ent) {
					let destination = exit.destination.format(hashmap!("{player}" => player.id.0.as_str()));
					emigration.emigrants.push((player.id.clone(), destination, exit.dest_pos.clone()));
					break;
				}
			}
		}
	}
}

