use crate::config::{Keys, Theme};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Config {
	pub theme: Theme,
	pub keys: Keys,
}
