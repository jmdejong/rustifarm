
use crate::{
	Pos,
	Encyclopedia,
	Template,
	Result,
	componentwrapper::PreEntity
};

#[derive(Default)]
pub struct NewEntities {
	pub to_build: Vec<(Pos, PreEntity)>,
	pub encyclopedia: Encyclopedia
}

impl NewEntities {
	pub fn new(encyclopedia: Encyclopedia) -> Self {
		Self{
			to_build: Vec::new(),
			encyclopedia
		}
	}
	pub fn create(&mut self, pos: Pos, template: Template) -> Result<()> {
		let components = self.encyclopedia.construct(&template)?;
		self.to_build.push((pos, components));
		Ok(())
	}
}
