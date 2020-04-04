

use specs::{
	ReadStorage,
	Write,
	Entities,
	System,
	Join
};

use crate::components::{
	Position,
	New,
	Player
};

use crate::resources::{
	Ground,
	Players
};


pub struct RegisterNew;
impl <'a> System<'a> for RegisterNew {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, New>,
		Write<'a, Ground>,
		ReadStorage<'a, Position>,
		Write<'a, Players>,
		ReadStorage<'a, Player>
	);
	fn run(&mut self, (entities, new, mut ground, positions, mut player_list, players): Self::SystemData) {
		for (ent, pos, _new) in (&entities, &positions, &new).join() {
			ground.insert(pos.pos, ent);
		}
		for (ent, player, _new) in (&entities, &players, &new).join(){
			player_list.entities.insert(player.id.clone(), ent);
		}
	}
}

