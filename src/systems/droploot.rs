
use rand::Rng;

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
		Trigger,
		TriggerBox,
		Flags,
		Flag
	},
	resources::{NewEntities, Ground},
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
		ReadStorage<'a, Flags>
	);
	
	fn run(&mut self, (positions, mut new, triggerboxes, loots, ground, flags): Self::SystemData) {
		for (position, triggerbox, loot) in (&positions, &triggerboxes, &loots).join(){
			if triggerbox.has_message(&[Trigger::Die, Trigger::Loot]) {
				for (template, chance) in &loot.loot {
					if *chance > rand::thread_rng().gen_range(0.0, 1.0) {
						// todo: better error handling
						let mut pos = position.pos;
						for t in &[(0,0), (1,0), (-1,0), (0,1), (0,-1)] {
							let p = pos + Pos::from_tuple(*t);
							let ground_flags = ground.flags_on(p, &flags);
							if !ground_flags.contains(&Flag::Blocking) && ground_flags.contains(&Flag::Floor) {
								pos = p;
								break;
							}
						}
						new.create(pos, &template).unwrap();
					}
				}
			}
		}
	}
}
