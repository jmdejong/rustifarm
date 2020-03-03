

use rand::Rng;

use specs::{
	ReadStorage,
	WriteStorage,
	Entities,
	System,
	Join
};

use crate::{
	components::{Controller, ControlCooldown, Fighter, MonsterAI, Home, Health, Position},
	controls::{Control, Direction::{self, North, South, East, West}},
	Pos
};


pub struct ControlAI;
impl <'a> System<'a> for ControlAI {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Controller>,
		ReadStorage<'a, ControlCooldown>,
		ReadStorage<'a, MonsterAI>,
		ReadStorage<'a, Fighter>,
		ReadStorage<'a, Home>,
		ReadStorage<'a, Health>,
		ReadStorage<'a, Position>
	);
	fn run(&mut self, (entities, mut controllers, cooldowns, ais, fighters, homes, healths, positions): Self::SystemData) {
	
		for (entity, ai, position, ()) in (&entities, &ais, &positions, !&cooldowns).join() {
			if let Some(fighter) = fighters.get(entity) {
				let mut closest_distance = ai.view_distance + 1;
				let mut closest = None;
				let mut closest_position = None;
				for (target, target_position, _) in (&entities, &positions, &healths).join() {
					if target == entity {
						continue;
					}
					let distance = position.pos.distance_to(target_position.pos);
					if distance < closest_distance {
						closest_distance = distance;
						closest = Some(target);
						closest_position = Some(target_position);
					}
				}
				if let Some(target) = closest {
					if closest_distance <= fighter.range {
						controllers.insert(entity, Controller{control: Control::AttackTarget(target)}).unwrap();
					} else {
						if let Some(direction) = step_to(position.pos, closest_position.unwrap().pos){
							controllers.insert(entity, Controller{control: Control::Move(direction)}).unwrap();
						}
					}
					return;
				}
			}
			if rand::thread_rng().gen_range(0.0, 1.0) < ai.move_chance {
				if let Some(home) = homes.get(entity) {
					if rand::thread_rng().gen_range(0.0, 1.0) < ai.homesickness * (position.pos.distance_to(home.home) as f64) {
						let direction = step_to(position.pos, home.home).unwrap();
						controllers.insert(entity, Controller{control: Control::Move(direction)}).unwrap();
						return;
					}
				}
				let direction = [North, South, East, West][rand::thread_rng().gen_range(0, 4)];
				controllers.insert(entity, Controller{control: Control::Move(direction)}).unwrap();
			}
		}
	}
}

fn step_to(p: Pos, t: Pos) -> Option<Direction> {
	let mut directions = Vec::new();
	if t.x > p.x {directions.push(East);}
	else if t.x < p.x {directions.push(West);}
	if t.y > p.y {directions.push(South);}
	else if t.y < p.y {directions.push(North);}
	if directions.is_empty() {
		None
	} else {
		Some(directions[rand::thread_rng().gen_range(0, directions.len())])
	}
}

