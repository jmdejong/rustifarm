
use std::collections::HashMap;
use serde_json::{json, Value};
use specs::{
	Component,
	HashMapStorage
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot {
	Hand,
	Body
}

impl Slot {
	pub fn from_str(txt: &str) -> Option<Self> {
		match txt {
			"hand" => Some(Self::Hand),
			"body" => Some(Self::Body),
			_ => None
		}
	}
	pub fn to_string(&self) -> String {
		match self {
			Self::Hand => "hand",
			Self::Body => "body"
		}.to_string()
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stat {
	Strength,
	Defence
}

impl Stat {
	pub fn from_str(txt: &str) -> Option<Self> {
		match txt {
			"strength" => Some(Self::Strength),
			"defence" => Some(Self::Defence),
			_ => None
		}
	}
	pub fn to_string(&self) -> String {
		match self {
			Self::Strength => "strength",
			Self::Defence => "defence"
		}.to_string()
	}
}


#[derive(Debug, Clone, PartialEq)]
pub struct Equippable {
	pub slot: Slot,
	pub stats: HashMap<Stat, i64>
}

impl Equippable {
	pub fn from_json(val: &Value) -> Option<Self> {
		Some(Equippable{
			slot: Slot::from_str(val.get("slot")?.as_str()?)?,
			stats: val
				.get("stats")?
				.as_object()?
				.into_iter()
				.map(|(k, v)| 
					Some((Stat::from_str(k.as_str())?, v.as_i64()?))
				)
				.collect::<Option<HashMap<Stat, i64>>>()?
		})
	}
	pub fn to_json(&self) -> Value {
		json!({
			"slot": self.slot.to_string(),
			"stats": self.stats.iter().map(|(k, v)| (k.to_string(), *v)).collect::<HashMap<String, i64>>()
		})
	}
}



#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Equipment {
	pub equipment: HashMap<Slot, Option<Equippable>>
}

impl Equipment {
	pub fn get_bonus(&self, stat: Stat) -> i64 {
		let mut bonus = 0;
		for v in self.equipment.values() {
			if let Some(equippable) = v {
				if let Some(s) = equippable.stats.get(&stat) {
					bonus += s;
				}
			}
		}
		bonus
	}
	pub fn all_bonuses(&self) -> HashMap<Stat, i64> {
		let mut bonuses: HashMap<Stat, i64> = HashMap::new();
		for v in self.equipment.values() {
			if let Some(equippable) = v {
				for (stat, s) in equippable.stats.iter(){
					let current: i64 = *bonuses.entry(*stat).or_insert(0);
					bonuses.insert(*stat, current + s);
				}
			}
		}
		bonuses
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::hashmap;
	
	
	#[test]
	fn slots() {
		assert_eq!(Slot::from_str("hand"), Some(Slot::Hand));
		assert_eq!(Slot::from_str("body"), Some(Slot::Body));
		assert_eq!(Slot::from_str("hands"), None);
		assert_eq!(Slot::from_str("head"), None);
	}
	
	#[test]
	fn stats() {
		assert_eq!(Stat::from_str("strength"), Some(Stat::Strength));
		assert_eq!(Stat::from_str("defence"), Some(Stat::Defence));
		assert_eq!(Stat::from_str("hand"), None);
		assert_eq!(Stat::from_str("body"), None);
		assert_eq!(Stat::from_str("attack"), None);
	}
	
	#[test]
	fn equippable_from_json() {
		assert_eq!(
			Equippable::from_json(&json!({"slot": "hand", "stats": {"strength": 10}})),
			Some(Equippable {slot: Slot::Hand, stats: hashmap!(Stat::Strength => 10)})
		);
	}
	
	
	#[test]
	fn bonus_value() {
		assert_eq!(
			Equipment{equipment: hashmap!(
				Slot::Hand => Some(Equippable{
					slot: Slot::Hand,
					stats: hashmap!(Stat::Strength => 15)
				}),
				Slot::Body => None
			)}.get_bonus(Stat::Strength),
			15
		);
	}
}
