
use rand::{Rng, seq::SliceRandom};

use specs::{
	ReadStorage,
	System,
	Join,
	Write,
	Read
};

use crate::{
	components::{
		Position,
		Loot,
		LootHolder,
		Trigger,
		TriggerBox,
		Flags,
		Flag,
		Removed
	},
	resources::{NewEntities, Ground},
	componentwrapper::ComponentWrapper,
	Pos
};


pub struct DropLoot;
impl <'a> System<'a> for DropLoot{
	type SystemData = (
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		ReadStorage<'a, TriggerBox>,
		ReadStorage<'a, Loot>,
		Read<'a, Ground>,
		ReadStorage<'a, Flags>,
		ReadStorage<'a, LootHolder>
	);
	
	fn run(&mut self, (positions, mut new, triggerboxes, loots, ground, flags, lootholders): Self::SystemData) {
		for (position, triggerbox, loot) in (&positions, &triggerboxes, &loots).join(){
			if triggerbox.has_message(&[Trigger::Die, Trigger::Loot]) {
				new.to_build.push((position.pos, vec![
					ComponentWrapper::LootHolder(LootHolder{loot: loot.loot.clone()}),
					ComponentWrapper::Removed(Removed)
				]));
			}
		}
		for (position, loot) in (&positions, &lootholders).join() {
			for (template, chance) in &loot.loot {
				if *chance > rand::thread_rng().gen_range(0.0, 1.0) {
					let pos = pick_position(position.pos, &ground, &flags);
					// todo: better error handling
					new.create(pos, &template).unwrap();
				}
			}
		}
	}
}

fn pick_position<'a>(pos: Pos, ground: &Read<'a, Ground>, flags: &ReadStorage<'a, Flags>) -> Pos {
	let ground_flags = ground.flags_on(pos, &flags);
	if !ground_flags.contains(&Flag::Blocking) && ground_flags.contains(&Flag::Floor) {
		return pos
	}
	let mut neighbours = vec![(1,0), (-1,0), (0,1), (0,-1)];
	neighbours.shuffle(&mut rand::thread_rng());
	for t in neighbours {
		let p = pos + Pos::from_tuple(t);
		let ground_flags = ground.flags_on(p, &flags);
		if !ground_flags.contains(&Flag::Blocking) && ground_flags.contains(&Flag::Floor) {
			return p
		}
	}
	return pos;
}
