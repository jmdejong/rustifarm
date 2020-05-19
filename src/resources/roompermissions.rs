
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct RoomPermissions {
	#[serde(default)]
	pub build: bool
}
