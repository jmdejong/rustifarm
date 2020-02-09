
use specs::{
	Read,
	Write,
	WriteStorage,
	System,
	Join,
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
		{
			let mut ents = Vec::new();
			for (ent, _new) in (&entities, &new).join() {
				ents.push(ent);
			}
			for ent in ents {
				new.remove(ent);
			}
		}
		for (pos, template) in &new_entities.templates {
			let mut builder = updater.create_entity(&entities);
			match new_entities.encyclopedia.construct(template) {
				Ok(comps) => {
					for comp in comps {
						builder = comp.build(builder);
					}
					builder.with(Position::new(*pos)).with(New).build();
				},
				Err(msg) => {println!("{}", msg);}
			}
		}
		new_entities.templates.clear();
	}
}
