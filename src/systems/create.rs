
use specs::{
	Read,
	Write,
	WriteStorage,
	System,
	Entities,
	LazyUpdate,
	Builder
};

use crate::components::{New, Position};
use crate::resources::{NewEntities};



pub struct Create;
impl <'a> System<'a> for Create {
	type SystemData = (
		Entities<'a>,
		Write<'a, NewEntities>,
		Read<'a, LazyUpdate>,
		WriteStorage<'a, New>
	);
	
	fn run(&mut self, (entities, mut new_entities, updater, mut new): Self::SystemData) {
		new.clear();
		for (pos, preentity) in &new_entities.to_build {
			let mut builder = updater.create_entity(&entities);
			for comp in preentity {
				builder = comp.build(builder);
			}
			builder.with(Position::new(*pos)).with(New).build();
		}
		new_entities.to_build.clear();
	}
}
