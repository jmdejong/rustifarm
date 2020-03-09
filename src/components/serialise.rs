
use specs::{
	Component,
	DenseVecStorage
};
use crate::{
	Template,
	componentwrapper::ComponentType
};


#[derive(Component, Debug, Clone)]
pub struct Serialise {
	pub template: Template,
	pub extract: Vec<(String, ComponentType, String)>
}
