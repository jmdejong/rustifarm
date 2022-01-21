
use std::collections::{HashMap, HashSet};

use crate::{
	components::{
		flags::Flag,
		Flags,
		Interactable,
		Serialise,
		RoomExit,
		Whitelist
	},
	RoomId,
	playerstate::RoomPos,
	componentwrapper::{ComponentWrapper, ComponentType},
	assemblage::DynamicAssemblage,
	parameter::Parameter,
	fromtoparameter::FromToParameter,
	template::Template,
	Result as AnyResult,
	PlayerId,
	aerr,
	hashset,
};
use super::basic::{MaybeVisible, TemplateSave};



#[derive(Debug, PartialEq, Clone)]
pub struct Portal;

impl DynamicAssemblage for Portal {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		
		let destination = arguments.get("destination")
			.and_then(<RoomId>::from_parameter)
			.ok_or(aerr!("no destination room found when instantiating {:?}", template))?;
		let dest_pos = arguments.get("destpos")
			.and_then(<String>::from_parameter)
			.unwrap_or("".to_string());
		Ok([
			MaybeVisible.instantiate(template, arguments)?,
			TemplateSave.instantiate(template, arguments)?,
			vec![
				ComponentWrapper::RoomExit(RoomExit{
					destination,
					dest_pos: if dest_pos.is_empty() {
						RoomPos::Unknown
					} else {
						RoomPos::Name(dest_pos)
					}
				}),
				ComponentWrapper::Flags(Flags( hashset!(Flag::Floor)))
			]
		].concat())
	}
}


#[derive(Debug, PartialEq, Clone)]
pub struct HomePortal;

impl DynamicAssemblage for HomePortal {
	
	fn instantiate(&self, template: &Template, arguments: &HashMap<String, Parameter>) -> AnyResult<Vec<ComponentWrapper>> {
		
		let allowed = arguments.get("allowed")
			.and_then(<HashMap<String, HashSet<PlayerId>>>::from_parameter)
			.unwrap_or_else(HashMap::new);
		let roomid = RoomId("_home+{player}".to_string());
		let mut components = [
			MaybeVisible.instantiate(template, arguments)?,
			TemplateSave.instantiate(template, arguments)?,
			vec![
				ComponentWrapper::RoomExit(RoomExit {
					destination: roomid.clone(),
					dest_pos: RoomPos::Unknown
				}),
				ComponentWrapper::Interactable(Interactable::Visit(roomid)),
				ComponentWrapper::Whitelist(Whitelist {
					allowed
				})
			]
		].concat();
		
		if template.should_save() {
			components.push(ComponentWrapper::Serialise(Serialise{
				template: template.clone(),
				extract: vec![("allowed".to_string(), ComponentType::Whitelist, "allowed".to_string())]
			}));
		}
		
		Ok(components)
	}
}


