use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
	pub scrolloff: u16,
}

impl Default for Settings {
	fn default() -> Self {
		Self { scrolloff: 8 }
	}
}
