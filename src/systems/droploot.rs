
use rand::Rng;

use specs::{
	ReadStorage,
	System,
	Join,
	Write
};

use crate::{
	components::{
		Position,
		Loot,
		Dead
	},
	resources::{NewEntities}
};


pub struct DropLoot;
impl <'a> System<'a> for DropLoot{
	type SystemData = (
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		ReadStorage<'a, Dead>,
		ReadStorage<'a, Loot>
	);
	
	fn run(&mut self, (positions, mut new, deads, loots): Self::SystemData) {
		for (position, _, loot) in (&positions, &deads, &loots).join(){
			for (template, chance) in &loot.loot {
				if *chance > rand::thread_rng().gen_range(0.0, 1.0) {
					// todo: better error handling
					new.create(position.pos, &template).unwrap();
				}
			}
		}
	}
}
