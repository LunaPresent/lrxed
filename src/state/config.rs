use crate::config::{Keys, Settings, Theme};

#[derive(Debug, Default, Clone)]
pub struct Config {
	pub theme: Theme,
	pub keys: Keys,
	pub settings: Settings,
}
