use serde::{Deserialize, Serialize};

use crate::config::{KeyMap, Settings, Theme};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
	pub theme: Theme,
	pub keys: KeyMap,
	pub settings: Settings,
}
