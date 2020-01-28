
use std::collections::HashMap;
use rand::Rng;
use specs::{
	VecStorage,
	Component,
	System,
	World,
	WorldExt,
	Builder,
	Join,
	ReadStorage,
	WriteStorage,
	DispatcherBuilder,
	Dispatcher,
	Write,
	EntityBuilder,
	Entity
};

use super::controls::Control;


// Components

#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[storage(VecStorage)]
struct Position {
	x: i32,
	y: i32
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct Visible {
    sprite: String,
    height: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Controller(Option<Control>);


// Resources

#[derive(Default)]
struct Size (i32, i32);

#[derive(Default)]
struct TopView {
	width: i32,
	height: i32,
	cells: HashMap<Position, Vec<Visible>>
}

// Systems

struct Draw;

impl <'a> System<'a> for Draw {
	
	type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Visible>, Write<'a, TopView>);
	
	fn run(&mut self, (pos, vis, mut view): Self::SystemData) {
		view.cells.clear();
		for (pos, vis) in (&pos, &vis).join(){
			if pos.x >= 0 && pos.y >= 0 && pos.x < view.width && pos.y < view.height {
				view.cells.entry(*pos).or_insert(Vec::new()).push(vis.clone());
				view.cells.get_mut(pos).unwrap().sort_by(|a, b| b.height.partial_cmp(&a.height).unwrap());
			}
		}
	}
}

// struct Control;
// impl <'a> System <'a> for Control {
// 	type SystemData = WriteStorage<'a, Controller>;
// 	fn run (&mut self, mut controller: Self::SystemData) {
// 		for controller in &mut controller.join()
// 	}
// }

struct Move;
impl <'a> System<'a> for Move {
	type SystemData = (WriteStorage<'a, Controller>, WriteStorage<'a, Position>);
	fn run(&mut self, (mut controller, mut pos): Self::SystemData) {
		for (controller, pos) in (&mut controller, &mut pos).join(){
			if let Some(control) = &controller.0 {
				match control {
					Control::Move(direction) => {
						let (dx, dy) = direction.to_position();
						pos.x += dx;
						pos.y += dy;
					}
					_ => {}
				}
				controller.0 = None
			}
		}
	}
}


// Higher level stuff

pub struct Room<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>,
	spawn: (i32, i32),
	players: HashMap<String, Entity>
}

impl <'a, 'b>Room<'a, 'b> {

	pub fn new(size: (i32, i32)) -> Room<'a, 'b> {
		let (width, height) = size;
		let mut world = World::new();
		world.register::<Position>();
		world.register::<Visible>();
		world.register::<Controller>();
		world.insert(Size(width, height));
		world.insert(TopView{width: width, height: height, cells: HashMap::new()});
		
		let dispatcher = DispatcherBuilder::new()
			.with(Draw, "draw", &[])
			.with(Move, "move", &["draw"])
			.build();
		
		let mut room = Room {
			world,
			dispatcher,
			spawn: (width / 2, height / 2),
			players: HashMap::new()
		};
		gen_room(&mut room);
		room
	}
	
	pub fn view(&self) -> (Vec<usize>, Vec<Vec<String>>) {
		let tv = &*self.world.fetch::<TopView>();
		let width = tv.width;
		let height = tv.height;
		let size = width * height;
		let mut values :Vec<usize> = Vec::with_capacity(size as usize);
		let mut mapping: Vec<Vec<String>> = Vec::new();
		for y in 0..height {
			for x in 0..width {
				let sprites: Vec<String> = match tv.cells.get(&Position{x: x, y: y}) {
					Some(sprites) => {sprites.iter().map(|v| v.sprite.clone()).collect()}
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
	
	pub fn add_obj(&mut self, template: &dyn Assemblage, (x, y): (i32, i32)) -> Entity {
		template.build(self.world.create_entity()).with(Position{x, y}).build()
	}
	
	pub fn add_player(&mut self, name: &str) {
		let ent = self.add_obj(&Player::new(name), self.spawn);
		self.players.insert(name.to_string(), ent);
	}
	
	pub fn remove_player(&mut self, name: &str){
		// todo: proper error handling
		let ent = self.players.remove(name).expect("unknown player name");
		self.world.delete_entity(ent).expect("player in world does not have entity");
	}
	
// 	pub fn clear_controls(&mut self){
// 		(*self.world.fetch_mut::<Controls>()).0.clear();
// 	}
	
	pub fn control(&mut self, name: String, control: Control){
		if let Some(ent) = self.players.get(&name){
			self.world.write_component::<Controller>().get_mut(*ent).unwrap().0 = Some(control);//.insert(*ent, Controller(control));
		}
	}
}

fn gen_room(room: &mut Room){
	
	let (width, height) = room.get_size();
	for x in 0..width {
		room.add_obj(&Wall, (x, 0));
		room.add_obj(&Wall, (x, height - 1));
	}
	for y in 1..height-1 {
		room.add_obj(&Wall, (0, y));
		room.add_obj(&Wall, (width - 1, y));
	}
	for x in 1..width-1 {
		for y in 1..height-1 {
			room.add_obj(&Grass::new(), (x, y));
		}
	}
}


pub trait Assemblage {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>;
}



// Entity types

struct Wall;

impl Assemblage for Wall {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
		builder.with(Visible{sprite: "wall".to_string(), height: 2.0})
	}
}

struct Grass {
	sprite: String
}

impl Grass {
	fn new() -> Grass {
		Grass {
			sprite: ["grass1", "grass2", "grass3", "grass1", "grass2", "grass3", "ground"][rand::thread_rng().gen_range(0,7)].to_string()
		}
	}
}

impl Assemblage for Grass {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
		builder.with(Visible{sprite: self.sprite.to_string(), height: 0.1})
	}
}


struct Player {
	name: String
}

impl Player {	
	fn new(name: &str) -> Player {
		Player { name: name.to_string()}
	}
}

impl Assemblage for Player {
	fn build<'a>(&self, builder: EntityBuilder<'a>) -> EntityBuilder<'a>{
			builder.with(Visible{sprite: "player".to_string(), height: 1.0}).with(Controller(None))
	}
}
