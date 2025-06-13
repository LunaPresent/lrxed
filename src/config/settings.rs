use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Settings {
	pub scrolloff: u16,
	pub default_path: Option<PathBuf>,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			scrolloff: 8,
			default_path: None,
		}
	}
}
