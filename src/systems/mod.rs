
mod controlinput;
mod registernew;
mod moving;
mod view;
mod remove;
mod create;
mod take;
mod migrate;
mod useitem;
mod attacking;
mod trapping;
mod fight;

pub use self::{
	controlinput::ControlInput,
	registernew::RegisterNew,
	moving::Move,
	view::View,
	remove::Remove,
	create::Create,
	take::Take,
	migrate::Migrate,
	useitem::Use,
	attacking::Attacking,
	trapping::Trapping,
	fight::Fight
};
