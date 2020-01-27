
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos(pub i32, pub i32);


pub struct Room {
	objects :HashMap<Pos, Vec<GameObject>>,
	pub area :(Pos, Pos),
	players :HashMap<String, GameObject>
}


impl Room {
	
	pub fn new(area :(Pos, Pos)) -> Room {
		Room {
			objects: HashMap::new(),
			area: area,
			players: HashMap::new()
		}
	}
	
	
	pub fn get_sprites(&self, pos :&Pos) -> Vec<String> {
		match self.objects.get(pos) {
			Some(objs) => {objs.iter().map(|o| o.sprite.clone()).collect()}
			None => {Vec::new()}
		}
	}
	
	pub fn add_obj(&mut self, pos :Pos, obj :GameObject) {
		let place = self.objects.entry(pos).or_insert(Vec::new());
		place.push(obj);
	}
	
// 		let mut sprites :Vec<String> = Vec::new();
// 		for maybe_obj in self.objects.get((x, y))
// 			sprites.push
		
	
	pub fn draw(&self) -> (Vec<usize>, Vec<Vec<String>>) {
		let (minp, maxp) = &self.area;
		let Pos(xmin, ymin) = *minp;
		let Pos(xmax, ymax) = *maxp;
		let width = xmax - xmin;
		let height = ymax - ymin;
		let size = width * height;
		let mut values :Vec<usize> = Vec::with_capacity(size as usize);
		let mut mapping = Vec::with_capacity(size as usize);
		for y in ymin..ymax {
			for x in xmin..xmax {
				values.push(mapping.len());
				mapping.push(self.get_sprites(&Pos(x, y)));
			}
		}
		return (values, mapping)
	}
}


#[derive(Clone)]
pub struct GameObject {
// 	pos :Pos,
// 	name :&str,
	sprite :String
}

impl GameObject {
	
	pub fn new(sprite :&str) -> GameObject {
		GameObject {
// 			pos: pos,
			sprite: sprite.to_string()
		}
	}
	
}
