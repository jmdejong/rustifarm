
use specs::{Component, DenseVecStorage};
use crate::{Template};

#[derive(Component, Debug, Clone)]
pub struct Item {
	pub ent: Template,
	pub name: String,
	pub action: ItemAction
}

#[derive(Debug, Clone)]
pub enum ItemAction {
	Eat{health: i64},
	Build(Template),
	None
}

