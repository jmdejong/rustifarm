
use specs::{
	Write,
	WriteStorage,
	ReadStorage,
	Entities,
	System,
	Join
};

use crate::{
	components::{Dead, Removed, Player},
	resources::Emigration,
	purgatory,
	playerstate::RoomPos
};


pub struct Die;
impl <'a> System<'a> for Die {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Dead>,
		WriteStorage<'a, Removed>,
		Write<'a, Emigration>,
		ReadStorage<'a, Player>
	);
	fn run(&mut self, (entities, deads, mut removeds, mut emigration, players): Self::SystemData) {
		// npcs etc get removed when dead
		for (entity, _, _) in (&entities, &deads, !&players).join() {
			removeds.insert(entity, Removed).unwrap();
		}
		// players move to purgatory when dead
		for (player, _) in (&players, &deads).join() {
			emigration.emigrants.push((player.id.clone(), purgatory::purgatory_id(), RoomPos::Unknown));
		}
	}
}

