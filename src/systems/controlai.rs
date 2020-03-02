

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
	controls::{Control, Direction::{North, South, East, West}}
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
						let p = position.pos;
						let t = closest_position.unwrap().pos;
						let mut directions = Vec::new();
						if t.x > p.x {directions.push(East);}
						else if t.x < p.x {directions.push(West);}
						if t.y > p.y {directions.push(South);}
						else if t.y < p.y {directions.push(North);}
						if !directions.is_empty() {
							let direction = directions[rand::thread_rng().gen_range(0, directions.len())];
							controllers.insert(entity, Controller{control: Control::Move(direction)}).unwrap();
						}
					}
					return;
				}
			}
			if rand::thread_rng().gen_range(0.0, 1.0) < ai.move_chance {
				let direction = [North, South, East, West][rand::thread_rng().gen_range(0, 4)];
				controllers.insert(entity, Controller{control: Control::Move(direction)}).unwrap();
// 				home = roomData.getComponent(obj, Home)
// 				if home is not None and home.home.inRoom() and random.random() < (ai.homesickness * pathfinding.distanceBetween(obj, home.home)):
// 					direction = pathfinding.stepTo(obj, home.home)
// 				else: 
// 					direction = random.choice(["north", "south", "east", "west"])
// 				movable.direction = direction
			}
		}
	}
}

