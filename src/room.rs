
use std::collections::HashMap;

use specs::{
	VecStorage,
	Component,
	System,
	World,
	WorldExt,
	Builder,
	Join,
	ReadStorage,
	DispatcherBuilder,
	Dispatcher,
	Write
};


// Components

#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[storage(VecStorage)]
struct Position {
	x: i32,
	y: i32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Visible {
    sprite: String,
    height: f32
}

#[derive(Default)]
struct Size (i32, i32);


// Resources

#[derive(Default)]
struct TopView {
	width: i32,
	height: i32,
	cells: HashMap<Position, Vec<String>>
}

// Systems

struct Draw;

impl <'a> System<'a> for Draw {
	
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Write<'a, TopView>);
	
	fn run(&mut self, (pos, vis, mut view): Self::SystemData) {
		view.cells.clear();
		for (pos, vis) in (&pos, &vis).join(){
			if pos.x >= 0 && pos.y >= 0 && pos.x < view.width && pos.y < view.height {
				view.cells.entry(*pos).or_insert(Vec::new()).push(vis.sprite.clone());
			}
		}
	}
}


// Higher level stuff

pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(size: (i32, i32)) -> Room<'a, 'b> {
		let (width, height) = size;
		let mut world = World::new();
		world.register::<Position>();
		world.register::<Visible>();
		world.insert(Size(width, height));
		world.insert(TopView{width: width, height: height, cells: HashMap::new()});
		
		let dispatcher = DispatcherBuilder::new()
			.with(Draw, "draw", &[])
			.build();
		
		gen_world(&mut world);
		
		Room {
			world,
			dispatcher
		}
	}
	
	pub fn view(&self) -> (Vec<usize>, Vec<Vec<String>>) {
		let tv = &*self.world.fetch::<TopView>();
		let width = tv.width;
		let height = tv.height;
		let size = width * height;
		let mut values :Vec<usize> = Vec::with_capacity(size as usize);
		let mut mapping: Vec<Vec<String>> = Vec::with_capacity(size as usize);
		for y in 0..height {
			for x in 0..width {
				let sprites = match tv.cells.get(&Position{x: x, y: y}) {
					Some(sprites) => {sprites.to_vec()}
					None => {vec![]}
				};
				values.push(
					match mapping.iter().position(|x| x == &sprites) {
						Some(index) => {
							index
						}
						None => {
							mapping.push(sprites);
							mapping.len() - 1
						}
					}
				)
			}
		}
		(values, mapping)
	}
	
	pub fn update(&mut self) {
		self.dispatcher.dispatch(&mut self.world);
		self.world.maintain();
	}
	
	pub fn get_size(&self) -> (i32, i32) {
		let Size(width, height) = *self.world.fetch::<Size>();
		(width, height)
	}
}

fn gen_world(world: &mut World){
	
	let Size(width, height) = *world.fetch::<Size>();
	for x in 0..width {
		world.create_entity().with(Position{x: x, y: 0}).with(Visible{sprite: "wall".to_string(), height: 1.0}).build();
		world.create_entity().with(Position{x: x, y: height - 1}).with(Visible{sprite: "wall".to_string(), height: 1.0}).build();
	}
	for y in 1..height-1 {
		world.create_entity().with(Position{x: 0, y: y}).with(Visible{sprite: "wall".to_string(), height: 1.0}).build();
		world.create_entity().with(Position{x: width - 1, y: y}).with(Visible{sprite: "wall".to_string(), height: 1.0}).build();
	}
}

