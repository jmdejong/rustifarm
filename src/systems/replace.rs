
use specs::{
	ReadStorage,
	System,
	Join,
	Write,
	WriteStorage,
	Entities
};

use crate::{
	components::{
		Position,
		Substitute,
		Removed,
		Serialise
	},
	resources::{NewEntities}
};


pub struct Replace;
impl <'a> System<'a> for Replace {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Position>,
		Write<'a, NewEntities>,
		ReadStorage<'a, Substitute>,
		WriteStorage<'a, Removed>,
		ReadStorage<'a, Serialise>
	);
	
	fn run(&mut self, (entities, positions, mut new, substitutes, mut removeds, serialisations): Self::SystemData) {
		for (entity, position, substitute, serialisation) in (&entities, &positions, &substitutes, (&serialisations).maybe()).join(){
			// todo: better error handling
			let mut template = substitute.into.clone();
			if let Some(serialise) = serialisation {
				// todo: extraction?
				template = template.merge(serialise.template.clone());
			}
			let preent = new.encyclopedia.construct(&template).unwrap();
			new.to_build.push((position.pos, preent));
			removeds.insert(entity, Removed).unwrap();
		}
	}
}
