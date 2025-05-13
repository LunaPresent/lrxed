use crate::config::{KeyMap, Settings, Theme};

#[derive(Debug, Default, Clone)]
pub struct Config {
	pub theme: Theme,
	pub keys: KeyMap,
	pub settings: Settings,
}
