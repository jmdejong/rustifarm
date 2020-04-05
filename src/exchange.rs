
use crate::{
	components::Inventory,
	ItemId,
	Encyclopedia
};

#[derive(Debug, Clone, PartialEq)]
pub struct Exchange {
	pub cost: Vec<ItemId>,
	pub offer: Vec<ItemId>
}

impl Exchange {
	pub fn show(&self) -> String {
		format!(
			"offer: [{}], price: [{}]",
			self.offer.iter().map(|i| i.0.clone()).collect::<Vec<String>>().join(", "),
			self.cost.iter().map(|i| i.0.clone()).collect::<Vec<String>>().join(", ")
		)
	}
	
	pub fn can_trade(&self, inventory: &Inventory) -> bool {
		if self.offer.len() as isize - self.cost.len() as isize > inventory.capacity as isize - inventory.items.len() as isize{
			return false;
		}
		let mut costs = self.cost.clone();
		for entry in inventory.items.iter() {
			if let Some(pos) = costs.iter().position(|x| *x == entry.itemid){
				costs.remove(pos);
			}
		}
		costs.is_empty()
	}
	
	pub fn trade(&self, inventory: &mut Inventory, enc: &Encyclopedia) {
		for item in self.cost.iter() {
			let pos = inventory.items.iter().position(|entry| entry.itemid == item.clone()).unwrap();
			inventory.items.remove(pos);
		}
		for item in self.offer.iter() {
			inventory.add_item(item.clone(), enc);
		}
	}
}
